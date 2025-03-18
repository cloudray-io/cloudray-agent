mod panic_error;
mod args;
mod config;
mod daemonise;
mod start;
mod cable;

use clap::Parser;
use std::sync::Arc;
use anyhow::anyhow;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use crate::args::Args;
use crate::config::Config;
use crate::daemonise::daemonise;
use crate::panic_error::PanicError;
use crate::start::start;

fn main() -> anyhow::Result<()> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let args = Args::parse();
    let config = Arc::new(Config::new(args));

    if config.wants_daemon() {
        if !config.can_daemon() {
            return Err(anyhow!(
                "--daemon option is not supported on this platform".to_string(),
            ));
        } else {
            daemonise()?;
        }
    }

    tokio_main(config)
}

#[tokio::main]
async fn tokio_main(config: Arc<Config>) -> anyhow::Result<()> {
    info!("Starting cloudray-agent…");
    start(config).await?;
    Ok(())
}
