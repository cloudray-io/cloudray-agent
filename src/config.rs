use crate::args::Args;
use crate::types::AgentToken;
use clap::Parser;
use std::sync::LazyLock;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::error;

const DEFAULT_ORIGIN_HOST: &str = "api.cloudray.io";

pub const METRICS_CPU_SAMPLE_EVERY: Duration = Duration::from_secs(5);
pub const METRICS_CPU_MAX_SAMPLES: usize = 12;
pub const METRICS_COLLECT_EVERY: Duration = Duration::from_secs(60);
pub const REPORT_EVERY: Duration = Duration::from_secs(60);
pub const REPORT_CHECK_EVERY: Duration = Duration::from_secs(2);
pub const RUNLOG_RUN_TIMEOUT: Duration = Duration::from_secs(3600);

static AGENT_TOKEN: LazyLock<RwLock<Option<AgentToken>>> = LazyLock::new(|| RwLock::new(None));
static SEND_REPORT_AT: LazyLock<RwLock<Instant>> = LazyLock::new(|| RwLock::new(Instant::now()));

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let args = Args::parse();
    Config::new(args)
});

pub struct Config {
    args: Args,
    origin_client: reqwest::Client,
}

impl Config {
    fn new(args: Args) -> Self {
        let origin_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| {
                error!("Failed to create custom reqwest::Client, falling back to default client");
                reqwest::Client::new()
            });
        Self {
            args,
            origin_client,
        }
    }

    pub fn reg_code(&self) -> &String {
        &self.args.reg_code
    }

    // Initialise the reqwest::Client only once and reuse it for each request for following benefits:
    // 1. Let reqwest keep the connection alive using Keep-Alive header (or HTTP/2?)
    // 2. Reduce the time SSL negotiation happens
    // 3. Reduce the number of DNS lookups
    pub fn origin_client(&self) -> &reqwest::Client {
        &self.origin_client
    }

    pub fn origin_host(&self) -> String {
        let default_host = &DEFAULT_ORIGIN_HOST.to_string();
        self.args
            .origin_host
            .as_ref()
            .unwrap_or(default_host)
            .to_string()
    }

    pub fn use_https(&self) -> bool {
        !self.origin_host().starts_with("localhost")
    }

    #[allow(dead_code)]
    pub fn cable_endpoint(&self) -> String {
        let host = self.origin_host();
        let scheme = if self.use_https() { "wss" } else { "ws" };
        format!("{}://{}/cable", scheme, host)
    }

    pub fn agent_v1_endpoint(&self) -> String {
        let host = self.origin_host();
        let scheme = if self.use_https() { "https" } else { "http" };
        format!("{}://{}/agent/v1/report", scheme, host)
    }

    #[inline]
    pub fn is_unix(&self) -> bool {
        cfg!(unix)
    }

    pub fn wants_daemon(&self) -> bool {
        self.args.daemon
    }

    pub fn can_daemon(&self) -> bool {
        self.is_unix()
    }
}

pub async fn get_agent_token() -> Option<AgentToken> {
    AGENT_TOKEN.read().await.clone()
}

pub async fn set_agent_token(token: AgentToken) {
    *AGENT_TOKEN.write().await = Some(token);
}

pub async fn get_send_report_at() -> Instant {
    *SEND_REPORT_AT.read().await
}

pub async fn set_send_report_at(instant: Instant) {
    *SEND_REPORT_AT.write().await = instant;
}

pub async fn set_send_report_at_now() {
    let now = Instant::now();
    // short-circuit to avoid a write lock
    if get_send_report_at().await <= now {
        return;
    }
    *SEND_REPORT_AT.write().await = Instant::now();
}
