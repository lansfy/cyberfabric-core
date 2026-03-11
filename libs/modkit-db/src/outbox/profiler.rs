//! Query profiler for the outbox pipeline.
//!
//! Enabled via the `outbox-profiler` feature flag.  Instruments all outbox
//! SQL operations and periodically emits a structured summary via `tracing`.
//!
//! # Output format
//!
//! ```text
//! Outbox Profile (window: 10.0s)
//!   enqueue.insert_body            calls=1204   total=  842.3ms  avg=  0.70ms  max=  12.4ms  avg_batch=  1.0  max_batch=1
//!   sequencer.claim_incoming       calls=96     total=  312.4ms  avg=  3.25ms  max=  24.8ms  avg_batch= 12.5  max_batch=50
//!   processor.read_messages        calls=512    total=  448.6ms  avg=  0.88ms  max=   8.3ms  avg_batch=  2.4  max_batch=50
//!   processor.sem_wait             calls=512    total=   42.1ms  avg=  0.08ms  max=   1.2ms  avg_batch=  0.0  max_batch=0
//!   vacuum.delete                  calls=4      total=   89.2ms  avg= 22.30ms  max=  31.4ms  avg_batch=  0.0  max_batch=0
//! ```

use std::fmt::Write as _;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Threshold above which a semaphore wait is considered "starvation" (50ms).
const STARVATION_THRESHOLD_US: u64 = 50_000;

/// All tracked outbox query operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    EnqueueInsertBody,
    EnqueueInsertIncoming,
    SequencerLockPartition,
    SequencerClaimIncoming,
    SequencerAllocateSeq,
    SequencerInsertOutgoing,
    ProcessorLockState,
    ProcessorReadMessages,
    ProcessorAck,
    VacuumDelete,
    /// Semaphore wait time before a processor can start work.
    /// High values indicate pool / concurrency starvation.
    SemaphoreWait,
}

impl Op {
    const ALL: [Op; 11] = [
        Op::EnqueueInsertBody,
        Op::EnqueueInsertIncoming,
        Op::SequencerLockPartition,
        Op::SequencerClaimIncoming,
        Op::SequencerAllocateSeq,
        Op::SequencerInsertOutgoing,
        Op::ProcessorLockState,
        Op::ProcessorReadMessages,
        Op::ProcessorAck,
        Op::VacuumDelete,
        Op::SemaphoreWait,
    ];

    fn label(self) -> &'static str {
        match self {
            Op::EnqueueInsertBody => "enqueue.insert_body",
            Op::EnqueueInsertIncoming => "enqueue.insert_incoming",
            Op::SequencerLockPartition => "sequencer.lock_partition",
            Op::SequencerClaimIncoming => "sequencer.claim_incoming",
            Op::SequencerAllocateSeq => "sequencer.allocate_seq",
            Op::SequencerInsertOutgoing => "sequencer.insert_outgoing",
            Op::ProcessorLockState => "processor.lock_state",
            Op::ProcessorReadMessages => "processor.read_messages",
            Op::ProcessorAck => "processor.ack",
            Op::VacuumDelete => "vacuum.delete",
            Op::SemaphoreWait => "processor.sem_wait",
        }
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Atomic counters for a single operation.
struct OpCounters {
    calls: AtomicU64,
    total_us: AtomicU64,
    max_us: AtomicU64,
    total_rows: AtomicU64,
    max_rows: AtomicU64,
}

impl OpCounters {
    const fn new() -> Self {
        Self {
            calls: AtomicU64::new(0),
            total_us: AtomicU64::new(0),
            max_us: AtomicU64::new(0),
            total_rows: AtomicU64::new(0),
            max_rows: AtomicU64::new(0),
        }
    }

    fn record(&self, elapsed_us: u64, rows: u64) {
        self.calls.fetch_add(1, Ordering::Relaxed);
        self.total_us.fetch_add(elapsed_us, Ordering::Relaxed);
        self.max_us.fetch_max(elapsed_us, Ordering::Relaxed);
        self.total_rows.fetch_add(rows, Ordering::Relaxed);
        self.max_rows.fetch_max(rows, Ordering::Relaxed);
    }

    fn snapshot_and_reset(&self) -> OpSnapshot {
        let calls = self.calls.swap(0, Ordering::Relaxed);
        let total_us = self.total_us.swap(0, Ordering::Relaxed);
        let max_us = self.max_us.swap(0, Ordering::Relaxed);
        let total_rows = self.total_rows.swap(0, Ordering::Relaxed);
        let max_rows = self.max_rows.swap(0, Ordering::Relaxed);
        OpSnapshot {
            calls,
            total_us,
            max_us,
            total_rows,
            max_rows,
        }
    }
}

/// A point-in-time snapshot of a single operation's counters.
#[derive(Debug, Clone, Copy)]
struct OpSnapshot {
    calls: u64,
    total_us: u64,
    max_us: u64,
    total_rows: u64,
    max_rows: u64,
}

impl OpSnapshot {
    #[allow(clippy::cast_precision_loss)]
    fn avg_ms(self) -> f64 {
        if self.calls == 0 {
            0.0
        } else {
            (self.total_us as f64 / self.calls as f64) / 1000.0
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn total_ms(self) -> f64 {
        self.total_us as f64 / 1000.0
    }

    #[allow(clippy::cast_precision_loss)]
    fn max_ms(self) -> f64 {
        self.max_us as f64 / 1000.0
    }

    #[allow(clippy::cast_precision_loss)]
    fn avg_rows(self) -> f64 {
        if self.calls == 0 {
            0.0
        } else {
            self.total_rows as f64 / self.calls as f64
        }
    }
}

/// Shared query profiler instance.
///
/// Create once when building the outbox pipeline. Calling [`record`](Self::record)
/// is lock-free (atomic increments only).
pub struct QueryProfiler {
    counters: [OpCounters; 11],
    /// Number of semaphore waits exceeding the starvation threshold.
    starvation_events: AtomicU64,
    window: Duration,
    /// Guards against double final-report emission. The reporter task
    /// (`spawn_reporter`) emits a final report on cancellation, then
    /// sets this flag. The Drop impl checks it to avoid a duplicate
    /// (which would always be empty since counters were already reset).
    final_report_emitted: AtomicBool,
}

impl QueryProfiler {
    /// Create a new profiler that emits a report every `window`.
    #[must_use]
    pub fn new(window: Duration) -> Arc<Self> {
        Arc::new(Self {
            counters: [
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
                OpCounters::new(),
            ],
            starvation_events: AtomicU64::new(0),
            window,
            final_report_emitted: AtomicBool::new(false),
        })
    }

    /// Record a completed query with optional row count.
    #[inline]
    pub fn record(&self, op: Op, elapsed: Duration, rows: u64) {
        #[allow(clippy::cast_possible_truncation)]
        let us = elapsed.as_micros() as u64;
        self.counters[op.index()].record(us, rows);
        // Track starvation events for semaphore waits
        if matches!(op, Op::SemaphoreWait) && us >= STARVATION_THRESHOLD_US {
            self.starvation_events.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Start measuring an operation. Returns a guard that records elapsed
    /// time on drop. Use `set_rows()` on the guard to record row count.
    #[inline]
    #[must_use]
    pub fn measure(&self, op: Op) -> MeasureGuard<'_> {
        MeasureGuard {
            profiler: self,
            op,
            start: Instant::now(),
            rows: 0,
        }
    }

    /// Snapshot all counters and reset. Returns the formatted report.
    ///
    /// One line per operation:
    /// `op_name  calls=N  total=Xms  avg=Yms  max=Zms  avg_batch=B  max_batch=M`
    fn snapshot_and_format(&self, window_secs: f64) -> Option<String> {
        let snapshots: Vec<(Op, OpSnapshot)> = Op::ALL
            .iter()
            .map(|&op| (op, self.counters[op.index()].snapshot_and_reset()))
            .collect();
        let starvation = self.starvation_events.swap(0, Ordering::Relaxed);

        let total_calls: u64 = snapshots.iter().map(|(_, s)| s.calls).sum();
        if total_calls == 0 {
            return None;
        }

        let mut report = String::with_capacity(512);
        #[allow(clippy::let_underscore_must_use)]
        let _ = writeln!(report, "Outbox Profile (window: {window_secs:.1}s)");

        for (op, snap) in &snapshots {
            if snap.calls > 0 {
                #[allow(clippy::let_underscore_must_use)]
                let _ = writeln!(
                    report,
                    "  {:<28} calls={:<6} total={:>9.1}ms  avg={:>6.2}ms  max={:>6.1}ms  avg_batch={:>5.1}  max_batch={}",
                    op.label(),
                    snap.calls,
                    snap.total_ms(),
                    snap.avg_ms(),
                    snap.max_ms(),
                    snap.avg_rows(),
                    snap.max_rows,
                );
            }
        }

        if starvation > 0 {
            #[allow(clippy::integer_division)]
            let threshold_ms = STARVATION_THRESHOLD_US / 1000;
            #[allow(clippy::let_underscore_must_use)]
            let _ = writeln!(
                report,
                "  *** POOL STARVATION: {starvation} semaphore wait(s) exceeded {threshold_ms}ms ***",
            );
        }

        Some(report)
    }

    /// Spawn the background reporter task. Returns a `JoinHandle` that runs
    /// until the `CancellationToken` is cancelled.
    pub fn spawn_reporter(
        self: &Arc<Self>,
        cancel: tokio_util::sync::CancellationToken,
    ) -> tokio::task::JoinHandle<()> {
        let profiler = Arc::clone(self);
        let window = profiler.window;
        tokio::spawn(async move {
            let window_secs = window.as_secs_f64();
            loop {
                tokio::select! {
                    () = cancel.cancelled() => break,
                    () = tokio::time::sleep(window) => {
                        if let Some(report) = profiler.snapshot_and_format(window_secs) {
                            tracing::info!(target: "outbox::profiler", "{report}");
                        }
                    }
                }
            }
            // Emit final report on shutdown
            if let Some(report) = profiler.snapshot_and_format(window_secs) {
                tracing::info!(target: "outbox::profiler", "Final report:{report}");
            }
            // Signal that final report was already emitted by the reporter task,
            // so the Drop impl doesn't emit a duplicate empty report.
            profiler.final_report_emitted.store(true, Ordering::Release);
        })
    }
}

impl Drop for QueryProfiler {
    fn drop(&mut self) {
        // Reporter task already emitted the final report — skip.
        if self.final_report_emitted.load(Ordering::Acquire) {
            return;
        }
        let window_secs = self.window.as_secs_f64();
        if let Some(report) = self.snapshot_and_format(window_secs) {
            tracing::info!(target: "outbox::profiler", "Final report:{report}");
        }
    }
}

/// RAII guard that records elapsed time on drop.
pub struct MeasureGuard<'a> {
    profiler: &'a QueryProfiler,
    op: Op,
    start: Instant,
    rows: u64,
}

impl MeasureGuard<'_> {
    /// Set the number of rows processed by this operation.
    #[inline]
    pub fn set_rows(&mut self, rows: u64) {
        self.rows = rows;
    }
}

impl Drop for MeasureGuard<'_> {
    fn drop(&mut self) {
        self.profiler
            .record(self.op, self.start.elapsed(), self.rows);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_snapshot() {
        let p = QueryProfiler::new(Duration::from_secs(10));
        p.record(Op::EnqueueInsertBody, Duration::from_micros(500), 1);
        p.record(Op::EnqueueInsertBody, Duration::from_micros(1500), 5);
        p.record(Op::SequencerClaimIncoming, Duration::from_millis(5), 100);

        let report = p.snapshot_and_format(10.0);
        assert!(report.is_some());
        let text = report.expect("report should be Some after recording ops");
        assert!(text.contains("enqueue.insert_body"));
        assert!(text.contains("sequencer.claim_incoming"));
        // Should not contain zero-call ops
        assert!(!text.contains("vacuum.delete"));

        // After snapshot, counters should be reset
        let report2 = p.snapshot_and_format(10.0);
        assert!(report2.is_none());
    }

    #[test]
    fn measure_guard() {
        let p = QueryProfiler::new(Duration::from_secs(10));
        {
            let _g = p.measure(Op::ProcessorAck);
            std::thread::sleep(Duration::from_millis(1));
        }
        let snap = p.counters[Op::ProcessorAck.index()].snapshot_and_reset();
        assert_eq!(snap.calls, 1);
        assert!(snap.total_us >= 1000); // at least 1ms
    }
}
