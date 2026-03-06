use serde::{Deserialize, Serialize};

use crate::module::DEFAULT_URL_PREFIX;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MiniChatConfig {
    #[serde(default = "default_url_prefix")]
    pub url_prefix: String,
    #[serde(default)]
    pub streaming: StreamingConfig,
    #[serde(default = "default_vendor")]
    pub vendor: String,
    #[serde(default)]
    pub estimation_budgets: EstimationBudgets,
    #[serde(default)]
    pub quota: QuotaConfig,
    #[serde(default)]
    pub outbox: OutboxConfig,
}

/// SSE streaming tuning parameters.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamingConfig {
    /// Bounded mpsc channel capacity between provider task and SSE writer.
    /// Valid range: 16–64 (default 32).
    #[serde(default = "default_channel_capacity")]
    pub sse_channel_capacity: u16,

    /// Ping keepalive interval in seconds.
    /// Valid range: 5–60 (default 15).
    #[serde(default = "default_ping_interval")]
    pub sse_ping_interval_seconds: u16,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            sse_channel_capacity: default_channel_capacity(),
            sse_ping_interval_seconds: default_ping_interval(),
        }
    }
}

impl StreamingConfig {
    /// Validate configuration values at startup. Returns an error message
    /// describing the first invalid value found.
    pub fn validate(self) -> Result<(), String> {
        if !(16..=64).contains(&self.sse_channel_capacity) {
            return Err(format!(
                "sse_channel_capacity must be 16-64, got {}",
                self.sse_channel_capacity
            ));
        }
        if !(5..=60).contains(&self.sse_ping_interval_seconds) {
            return Err(format!(
                "sse_ping_interval_seconds must be 5-60, got {}",
                self.sse_ping_interval_seconds
            ));
        }
        Ok(())
    }
}

fn default_channel_capacity() -> u16 {
    32
}

fn default_ping_interval() -> u16 {
    15
}

impl Default for MiniChatConfig {
    fn default() -> Self {
        Self {
            url_prefix: default_url_prefix(),
            streaming: StreamingConfig::default(),
            vendor: default_vendor(),
            estimation_budgets: EstimationBudgets::default(),
            quota: QuotaConfig::default(),
            outbox: OutboxConfig::default(),
        }
    }
}

/// Token estimation parameters sourced from `ConfigMap` (P1).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EstimationBudgets {
    #[serde(default = "default_bytes_per_token")]
    pub bytes_per_token_conservative: u32,
    #[serde(default = "default_fixed_overhead")]
    pub fixed_overhead_tokens: u32,
    #[serde(default = "default_safety_margin")]
    pub safety_margin_pct: u32,
    #[serde(default = "default_image_budget")]
    pub image_token_budget: u32,
    #[serde(default = "default_tool_surcharge")]
    pub tool_surcharge_tokens: u32,
    #[serde(default = "default_web_surcharge")]
    pub web_search_surcharge_tokens: u32,
    #[serde(default = "default_min_gen_floor")]
    pub minimal_generation_floor: u32,
}

impl Default for EstimationBudgets {
    fn default() -> Self {
        Self {
            bytes_per_token_conservative: default_bytes_per_token(),
            fixed_overhead_tokens: default_fixed_overhead(),
            safety_margin_pct: default_safety_margin(),
            image_token_budget: default_image_budget(),
            tool_surcharge_tokens: default_tool_surcharge(),
            web_search_surcharge_tokens: default_web_surcharge(),
            minimal_generation_floor: default_min_gen_floor(),
        }
    }
}

impl EstimationBudgets {
    pub fn validate(self) -> Result<(), String> {
        if self.bytes_per_token_conservative == 0 {
            return Err("bytes_per_token_conservative must be > 0".to_owned());
        }
        if self.minimal_generation_floor == 0 {
            return Err("minimal_generation_floor must be > 0".to_owned());
        }
        Ok(())
    }
}

fn default_bytes_per_token() -> u32 {
    4
}
fn default_fixed_overhead() -> u32 {
    100
}
fn default_safety_margin() -> u32 {
    10
}
fn default_image_budget() -> u32 {
    1000
}
fn default_tool_surcharge() -> u32 {
    500
}
fn default_web_surcharge() -> u32 {
    500
}
fn default_min_gen_floor() -> u32 {
    50
}

/// Quota enforcement configuration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuotaConfig {
    #[serde(default = "default_overshoot_tolerance")]
    pub overshoot_tolerance_factor: f64,
}

impl Default for QuotaConfig {
    fn default() -> Self {
        Self {
            overshoot_tolerance_factor: default_overshoot_tolerance(),
        }
    }
}

impl QuotaConfig {
    pub fn validate(self) -> Result<(), String> {
        if !(1.0..=1.5).contains(&self.overshoot_tolerance_factor) {
            return Err(format!(
                "overshoot_tolerance_factor must be 1.0-1.5, got {}",
                self.overshoot_tolerance_factor
            ));
        }
        Ok(())
    }
}

/// Outbox configuration for usage event publishing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutboxConfig {
    /// Queue name for usage events.
    #[serde(default = "default_outbox_queue_name")]
    pub queue_name: String,

    /// Number of outbox partitions. Must be 1–64.
    #[serde(default = "default_outbox_num_partitions")]
    pub num_partitions: u32,
}

impl Default for OutboxConfig {
    fn default() -> Self {
        Self {
            queue_name: default_outbox_queue_name(),
            num_partitions: default_outbox_num_partitions(),
        }
    }
}

impl OutboxConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.queue_name.trim().is_empty() {
            return Err("outbox queue_name must not be empty".to_owned());
        }
        if !(1..=64).contains(&self.num_partitions) {
            return Err(format!(
                "outbox num_partitions must be 1-64, got {}",
                self.num_partitions
            ));
        }
        Ok(())
    }
}

fn default_outbox_queue_name() -> String {
    "mini-chat.usage_snapshot".to_owned()
}

fn default_outbox_num_partitions() -> u32 {
    4
}

fn default_overshoot_tolerance() -> f64 {
    1.10
}

fn default_url_prefix() -> String {
    DEFAULT_URL_PREFIX.to_owned()
}

fn default_vendor() -> String {
    "hyperspot".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        StreamingConfig::default().validate().unwrap();
        EstimationBudgets::default().validate().unwrap();
        QuotaConfig::default().validate().unwrap();
        OutboxConfig::default().validate().unwrap();
    }

    #[test]
    fn estimation_budgets_validation() {
        let valid = EstimationBudgets::default();

        assert!(
            (EstimationBudgets {
                bytes_per_token_conservative: 0,
                ..valid
            })
            .validate()
            .is_err()
        );
        assert!(
            (EstimationBudgets {
                minimal_generation_floor: 0,
                ..valid
            })
            .validate()
            .is_err()
        );
    }

    #[test]
    fn quota_config_validation() {
        assert!(
            (QuotaConfig {
                overshoot_tolerance_factor: 0.99
            })
            .validate()
            .is_err()
        );
        assert!(
            (QuotaConfig {
                overshoot_tolerance_factor: 1.0
            })
            .validate()
            .is_ok()
        );
        assert!(
            (QuotaConfig {
                overshoot_tolerance_factor: 1.5
            })
            .validate()
            .is_ok()
        );
        assert!(
            (QuotaConfig {
                overshoot_tolerance_factor: 1.51
            })
            .validate()
            .is_err()
        );
    }

    #[test]
    fn channel_capacity_boundaries() {
        let valid = StreamingConfig::default();

        assert!(
            (StreamingConfig {
                sse_channel_capacity: 15,
                ..valid
            })
            .validate()
            .is_err()
        );
        assert!(
            (StreamingConfig {
                sse_channel_capacity: 16,
                ..valid
            })
            .validate()
            .is_ok()
        );
        assert!(
            (StreamingConfig {
                sse_channel_capacity: 64,
                ..valid
            })
            .validate()
            .is_ok()
        );
        assert!(
            (StreamingConfig {
                sse_channel_capacity: 65,
                ..valid
            })
            .validate()
            .is_err()
        );
    }

    #[test]
    fn ping_interval_boundaries() {
        let valid = StreamingConfig::default();

        assert!(
            (StreamingConfig {
                sse_ping_interval_seconds: 4,
                ..valid
            })
            .validate()
            .is_err()
        );
        assert!(
            (StreamingConfig {
                sse_ping_interval_seconds: 5,
                ..valid
            })
            .validate()
            .is_ok()
        );
        assert!(
            (StreamingConfig {
                sse_ping_interval_seconds: 60,
                ..valid
            })
            .validate()
            .is_ok()
        );
        assert!(
            (StreamingConfig {
                sse_ping_interval_seconds: 61,
                ..valid
            })
            .validate()
            .is_err()
        );
    }
}
