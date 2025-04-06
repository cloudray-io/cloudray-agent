use crate::args::Args;
use crate::types::AgentToken;
use clap::Parser;
use std::sync::LazyLock;
use tokio::sync::RwLock;

const ORIGIN_HOST: &str = "api.cloudray.io";

pub static AGENT_TOKEN: LazyLock<RwLock<Option<AgentToken>>> = LazyLock::new(|| RwLock::new(None));

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let args = Args::parse();
    Config::new(args)
});

pub struct Config {
    args: Args,
}

impl Config {
    fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn reg_code(&self) -> &String {
        &self.args.reg_code
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

    fn agent_v1_endpoint(&self) -> String {
        let host = self.origin_host();
        let scheme = if self.use_https() { "https" } else { "http" };
        format!("{}://{}/agent/v1", scheme, host)
    }

    pub fn agent_v1_handshake_endpoint(&self) -> String {
        self.agent_v1_endpoint() + "/handshake"
    }

    pub fn agent_v1_talk_endpoint(&self) -> String {
        self.agent_v1_endpoint() + "/talk"
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
