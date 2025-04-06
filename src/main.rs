mod args;
mod cable;
mod config;
mod daemonise;
mod device_uid;
mod experiment_talk_api;
mod experiment_ws;
mod generated;
mod message_queue;
mod net;
mod o2a_messages;
mod panic_error;
mod types;
mod utils;
mod version;

use crate::args::Args;
use crate::config::Config;
use crate::daemonise::daemonise;
use crate::experiment_talk_api::start;
use anyhow::anyhow;
use clap::Parser;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

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
