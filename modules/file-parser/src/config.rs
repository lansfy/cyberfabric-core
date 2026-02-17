use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Configuration for the `file_parser` module
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileParserConfig {
    #[serde(default = "default_max_file_size_mb")]
    pub max_file_size_mb: u64,

    /// Base directory for local file parsing (**required at runtime**). Only
    /// files under this directory (after symlink resolution / canonicalization)
    /// are allowed.  The module will fail to start if this field is missing or
    /// the path cannot be resolved.
    ///
    /// Wrapped in `Option` because the `ModKit` config loader requires `Default`,
    /// but `init()` treats `None` as a hard startup error.
    #[serde(default)]
    pub allowed_local_base_dir: Option<PathBuf>,
}

impl Default for FileParserConfig {
    fn default() -> Self {
        Self {
            max_file_size_mb: default_max_file_size_mb(),
            // None here — init() will reject this with a clear error message.
            allowed_local_base_dir: None,
        }
    }
}

fn default_max_file_size_mb() -> u64 {
    100
}
