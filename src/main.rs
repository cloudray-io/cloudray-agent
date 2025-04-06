mod args;
mod cable;
mod config;
mod daemonise;
mod device_uid;
mod experiment_talk_api;
mod experiment_ws;
mod generated;
mod machine_name;
mod message_queue;
mod net;
mod o2a_messages;
mod panic_error;
mod types;
mod utils;
mod version;

use crate::config::CONFIG;
use crate::daemonise::daemonise;
use crate::experiment_talk_api::start;
use anyhow::anyhow;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> anyhow::Result<()> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    if CONFIG.wants_daemon() {
        if !CONFIG.can_daemon() {
            return Err(anyhow!(
                "--daemon option is not supported on this platform".to_string(),
            ));
        } else {
            daemonise()?;
        }
    }

    tokio_main()
}

#[tokio::main]
async fn tokio_main() -> anyhow::Result<()> {
    info!("Starting cloudray-agent…");
    start().await?;
    Ok(())
}
