use crate::args::Args;
use crate::types::AgentToken;
use clap::Parser;
use std::sync::LazyLock;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::error;

const ORIGIN_HOST: &str = "api.cloudray.io";

pub static AGENT_TOKEN: LazyLock<RwLock<Option<AgentToken>>> = LazyLock::new(|| RwLock::new(None));

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
            .unwrap_or({
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
        let default_host = &ORIGIN_HOST.to_string();
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
