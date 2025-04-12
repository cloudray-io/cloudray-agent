mod args;
mod cable;
mod config;
mod daemonise;
mod device_uid;
mod experiment_ws;
mod generated;
mod machine_name;
mod message_queue;
mod net;
mod o2a_messages;
mod panic_error;
mod tasks;
mod types;
mod utils;
mod version;

use crate::config::CONFIG;
use crate::daemonise::daemonise;
use crate::tasks::metrics::run_metrics_task;
use crate::tasks::report::run_report_task;
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
    let report_task = run_report_task().await;
    run_metrics_task().await;

    let _ = report_task.await?;

    Ok(())
}
