pub mod args;
#[allow(clippy::module_inception)]
pub mod config;
pub mod global_env_file;

use anyhow::anyhow;
pub use config::*;
use std::path::PathBuf;

pub fn get_executable_path() -> anyhow::Result<PathBuf> {
    std::env::current_exe().map_err(|e| anyhow!("Failed to get current executable path: {}", e))
}
