use std::sync::Arc;

use sea_orm::{ConnectionTrait, DbBackend, Statement, TransactionTrait};
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};

use super::dialect::Dialect;
use super::types::{MaintenanceConfig, OutboxError};
use crate::Db;

/// Max rows per bounded vacuum chunk (SELECT + DELETE).
const VACUUM_BATCH_SIZE: usize = 10_000;

/// SQL LIMIT value for vacuum batch size.
const VACUUM_BATCH_LIMIT: i64 = 10_000;

/// Page size for the dirty-partition cursor.
const DIRTY_PAGE_SIZE: usize = 64;

/// SQL LIMIT value for dirty-partition page size.
const DIRTY_PAGE_LIMIT: i64 = 64;

/// Standalone vacuum background task that garbage-collects processed
/// outgoing rows and their associated body rows.
///
/// Counter-driven: only visits partitions where the processor has
/// bumped `modkit_outbox_vacuum_counter` since the last vacuum.
///
/// Each sweep snapshots all dirty partitions, drains each one
/// (delete chunks until `deleted < VACUUM_BATCH_SIZE`), decrements
/// the counter by the snapshot value, then sleeps for `vacuum_cooldown`.
/// Partitions dirtied during the sweep are picked up in the next cycle.
///
/// Resilient to transient DB errors: a failed snapshot or per-partition
/// error is logged and the sweep continues (or retries after cooldown).
/// The vacuum never kills itself on a transient failure.
pub struct VacuumTask {
    maintenance_sem: Arc<Semaphore>,
    config: MaintenanceConfig,
    #[cfg(feature = "outbox-profiler")]
    profiler: Option<Arc<super::profiler::QueryProfiler>>,
}

impl VacuumTask {
    pub fn new(maintenance_sem: Arc<Semaphore>, config: MaintenanceConfig) -> Self {
        Self {
            maintenance_sem,
            config,
            #[cfg(feature = "outbox-profiler")]
            profiler: None,
        }
    }

    #[cfg(feature = "outbox-profiler")]
    pub fn set_profiler(&mut self, profiler: Arc<super::profiler::QueryProfiler>) {
        self.profiler = Some(profiler);
    }

    /// Run the vacuum loop until cancellation.
    ///
    /// Each cycle: snapshot dirty partitions → drain each → sleep cooldown.
    /// Transient errors are logged and retried next cycle — the vacuum
    /// never exits on a recoverable failure.
    pub async fn run(self, db: &Db, cancel: CancellationToken) -> Result<(), OutboxError> {
        let cooldown = self.config.vacuum_cooldown;

        let (backend, dialect) = {
            let sea_conn = db.sea_internal();
            let b = sea_conn.get_database_backend();
            (b, Dialect::from(b))
        };

        loop {
            let sweep_start = tokio::time::Instant::now();

            // Phase 1: Snapshot all dirty partitions via paginated cursor.
            let dirty = match Self::snapshot_dirty(db, backend, &dialect, &cancel).await {
                Ok(d) => d,
                Err(e) => {
                    if cancel.is_cancelled() {
                        break;
                    }
                    warn!(error = %e, "vacuum: failed to snapshot dirty partitions, retrying after cooldown");
                    Self::sleep_remaining(cooldown, sweep_start, &cancel).await;
                    continue;
                }
            };

            if cancel.is_cancelled() {
                break;
            }

            // Phase 2: Drain each partition, then decrement its counter.
            let mut errors = 0u32;
            for (partition_id, snapshot_counter) in &dirty {
                if cancel.is_cancelled() {
                    break;
                }

                if let Err(e) = self
                    .drain_partition(
                        db,
                        backend,
                        &dialect,
                        *partition_id,
                        *snapshot_counter,
                        &cancel,
                    )
                    .await
                {
                    warn!(
                        partition_id,
                        error = %e,
                        "vacuum: failed to drain partition, skipping",
                    );
                    errors += 1;
                }
            }

            let elapsed = sweep_start.elapsed();
            debug!(
                partitions = dirty.len(),
                errors,
                elapsed_ms = u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX),
                "vacuum: sweep complete",
            );

            // Phase 3: Sleep for remaining cooldown (cooldown minus sweep time).
            Self::sleep_remaining(cooldown, sweep_start, &cancel).await;
        }

        Ok(())
    }

    /// Drain a single partition and decrement its counter.
    /// Extracted so the caller can catch errors per-partition.
    async fn drain_partition(
        &self,
        db: &Db,
        backend: DbBackend,
        dialect: &Dialect,
        partition_id: i64,
        snapshot_counter: i64,
        cancel: &CancellationToken,
    ) -> Result<(), OutboxError> {
        self.vacuum_partition(db, backend, dialect, partition_id, cancel)
            .await?;

        // Decrement counter by snapshot value. GREATEST(counter - snapshot, 0)
        // preserves any concurrent bumps from the processor.
        let conn = db.sea_internal();
        conn.execute(Statement::from_sql_and_values(
            backend,
            dialect.decrement_vacuum_counter(),
            [snapshot_counter.into(), partition_id.into()],
        ))
        .await?;

        Ok(())
    }

    /// Sleep for the remaining cooldown after a sweep, cancellation-aware.
    async fn sleep_remaining(
        cooldown: std::time::Duration,
        sweep_start: tokio::time::Instant,
        cancel: &CancellationToken,
    ) {
        let remaining = cooldown.saturating_sub(sweep_start.elapsed());
        if !remaining.is_zero() {
            tokio::select! {
                () = cancel.cancelled() => {}
                () = tokio::time::sleep(remaining) => {}
            }
        }
    }

    /// Collect all dirty partitions (counter > 0) via paginated cursor.
    /// Returns `(partition_id, counter)` pairs, snapshot taken once per sweep.
    async fn snapshot_dirty(
        db: &Db,
        backend: DbBackend,
        dialect: &Dialect,
        cancel: &CancellationToken,
    ) -> Result<Vec<(i64, i64)>, OutboxError> {
        let mut dirty = Vec::new();
        let mut cursor: i64 = 0;

        loop {
            if cancel.is_cancelled() {
                break;
            }

            let conn = db.sea_internal();
            let page = DIRTY_PAGE_LIMIT;
            let rows = conn
                .query_all(Statement::from_sql_and_values(
                    backend,
                    dialect.fetch_dirty_partitions(),
                    [cursor.into(), page.into()],
                ))
                .await?;

            if rows.is_empty() {
                break;
            }

            for r in &rows {
                let pid: i64 = r.try_get_by_index(0).map_err(|e| {
                    OutboxError::Database(sea_orm::DbErr::Custom(format!(
                        "partition_id column: {e}"
                    )))
                })?;
                let counter: i64 = r.try_get_by_index(1).map_err(|e| {
                    OutboxError::Database(sea_orm::DbErr::Custom(format!("counter column: {e}")))
                })?;
                dirty.push((pid, counter));
            }

            cursor = dirty.last().map_or(cursor, |&(pid, _)| pid);

            if rows.len() < DIRTY_PAGE_SIZE {
                break;
            }
        }

        Ok(dirty)
    }

    /// Drain a single partition: read `processed_seq`, then delete all
    /// outgoing + body rows with `seq <= processed_seq` in bounded chunks
    /// until `deleted < VACUUM_BATCH_SIZE`.
    async fn vacuum_partition(
        &self,
        db: &Db,
        backend: DbBackend,
        dialect: &Dialect,
        partition_id: i64,
        cancel: &CancellationToken,
    ) -> Result<(), OutboxError> {
        // Read processed_seq (PK lookup, cheap).
        let row = {
            let conn = db.sea_internal();
            conn.query_one(Statement::from_sql_and_values(
                backend,
                dialect.read_processor(),
                [partition_id.into()],
            ))
            .await?
        };

        let Some(row) = row else {
            return Ok(());
        };
        let processed_seq: i64 = row.try_get_by_index(0).map_err(|e| {
            OutboxError::Database(sea_orm::DbErr::Custom(format!(
                "`processed_seq` column: {e}",
            )))
        })?;
        if processed_seq == 0 {
            return Ok(());
        }

        let vacuum_sql = dialect.vacuum_cleanup();

        // Delete in bounded chunks until drained.
        loop {
            if cancel.is_cancelled() {
                break;
            }

            // Acquire maintenance permit, cancellation-aware.
            let _maint_permit = tokio::select! {
                () = cancel.cancelled() => break,
                result = self.maintenance_sem.acquire() => match result {
                    Ok(permit) => permit,
                    Err(_) => break,
                }
            };

            #[cfg(feature = "outbox-profiler")]
            let mut delete_guard = self
                .profiler
                .as_ref()
                .map(|p| p.measure(super::profiler::Op::VacuumDelete));

            let deleted = Self::delete_chunk(
                db,
                backend,
                dialect,
                &vacuum_sql,
                partition_id,
                processed_seq,
            )
            .await?;

            #[cfg(feature = "outbox-profiler")]
            if let Some(g) = delete_guard.as_mut() {
                g.set_rows(u64::try_from(deleted).unwrap_or(u64::MAX));
            }

            // _maint_permit drops here → permit released.

            if deleted < VACUUM_BATCH_SIZE {
                break; // Partition drained.
            }
        }

        Ok(())
    }

    /// Execute one bounded chunk of cleanup for a single partition.
    /// Returns the number of outgoing rows deleted.
    async fn delete_chunk(
        db: &Db,
        backend: DbBackend,
        dialect: &Dialect,
        vacuum_sql: &super::dialect::VacuumSql,
        partition_id: i64,
        processed_seq: i64,
    ) -> Result<usize, OutboxError> {
        let conn = db.sea_internal();
        let txn = conn.begin().await?;

        let limit = VACUUM_BATCH_LIMIT;

        let rows = txn
            .query_all(Statement::from_sql_and_values(
                backend,
                vacuum_sql.select_outgoing_chunk,
                [partition_id.into(), processed_seq.into(), limit.into()],
            ))
            .await?;

        if rows.is_empty() {
            txn.rollback().await?;
            return Ok(0);
        }

        let mut outgoing_ids: Vec<i64> = Vec::with_capacity(rows.len());
        let mut body_ids: Vec<i64> = Vec::with_capacity(rows.len());
        for r in &rows {
            let oid: i64 = r.try_get_by_index(0).map_err(|e| {
                OutboxError::Database(sea_orm::DbErr::Custom(format!("outgoing_id column: {e}")))
            })?;
            outgoing_ids.push(oid);
            if let Ok(bid) = r.try_get_by_index::<i64>(1) {
                body_ids.push(bid);
            }
        }

        let count = outgoing_ids.len();

        // DELETE outgoing rows by ID.
        if !outgoing_ids.is_empty() {
            let delete_sql = dialect.build_delete_outgoing_batch(outgoing_ids.len());
            let values: Vec<sea_orm::Value> = outgoing_ids.iter().map(|&id| id.into()).collect();
            txn.execute(Statement::from_sql_and_values(backend, &delete_sql, values))
                .await?;
        }

        // DELETE body rows by ID.
        if !body_ids.is_empty() {
            let delete_sql = dialect.build_delete_body_batch(body_ids.len());
            let values: Vec<sea_orm::Value> = body_ids.iter().map(|&id| id.into()).collect();
            txn.execute(Statement::from_sql_and_values(backend, &delete_sql, values))
                .await?;
        }

        txn.commit().await?;
        Ok(count)
    }
}
