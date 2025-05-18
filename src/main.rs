mod cable;
mod config;
mod device_uid;
mod experiment_ws;
mod generated;
mod install;
mod machine_name;
mod message_queue;
mod net;
mod o2a_messages;
mod os;
mod tasks;
mod types;
mod utils;
mod version;

use crate::config::args::Commands;
use crate::config::global_env_file;
use crate::config::CONFIG;
use crate::install::service::install_service;
use crate::tasks::metrics::run_metrics_task;
use crate::tasks::report::run_report_task;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> anyhow::Result<()> {
    global_env_file::read_global_env()?;
    setup_logging();

    match CONFIG.command() {
        Commands::InstallService {
            reg_code,
            common_args,
        } => install_service(reg_code.clone(), common_args.origin_host.clone()),
        Commands::Run { .. } => tokio_main(),
    }
}

#[tokio::main]
async fn tokio_main() -> anyhow::Result<()> {
    info!("Starting cloudray-agent…");
    let report_task = run_report_task().await;
    run_metrics_task().await;

    let _ = report_task.await?;

    Ok(())
}

fn setup_logging() {
    // In debug mode, show full module path
    #[cfg(debug_assertions)]
    {
        fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
            )
            .init();
    }

    // In release mode, set log level to info, and hide module path
    #[cfg(not(debug_assertions))]
    {
        fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
            )
            .with_target(false)
            .init();
    }
}
