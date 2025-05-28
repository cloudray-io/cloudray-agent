use crate::config::args::{Args, Commands};
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

pub const DEFAULT_ENV_FILE_PATH: &str = "/etc/cloudray-agent/environment";

static AGENT_TOKEN: LazyLock<RwLock<Option<AgentToken>>> = LazyLock::new(|| RwLock::new(None));
static SEND_REPORT_AT: LazyLock<RwLock<Instant>> = LazyLock::new(|| RwLock::new(Instant::now()));

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let args = Args::parse();
    Config::new(args)
});

pub struct Config {
    args: Args,
    reg_code: Option<String>,
    origin_host: String,
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

        let (reg_code, origin_host) = match &args.command {
            Commands::InstallService {
                reg_code,
                common_args,
                ..
            } => (Some(reg_code.clone()), common_args.origin_host.clone()),
            Commands::Run {
                reg_code,
                common_args,
                ..
            } => (reg_code.clone(), common_args.origin_host.clone()),
        };

        let mut origin_host = origin_host.unwrap_or(DEFAULT_ORIGIN_HOST.to_string());

        origin_host = if origin_host.starts_with("http://") || origin_host.starts_with("https://") {
            origin_host
        } else {
            format!("https://{}", origin_host)
        };

        Self {
            args,
            reg_code,
            origin_host,
            origin_client,
        }
    }

    pub fn reg_code(&self) -> Option<String> {
        self.reg_code.clone()
    }

    // Initialise the reqwest::Client only once and reuse it for each request for following benefits:
    // 1. Let reqwest keep the connection alive using Keep-Alive header (or HTTP/2?)
    // 2. Reduce the time SSL negotiation happens
    // 3. Reduce the number of DNS lookups
    pub fn origin_client(&self) -> &reqwest::Client {
        &self.origin_client
    }

    pub fn origin_host(&self) -> String {
        self.origin_host.clone()
    }

    #[allow(dead_code)]
    pub fn cable_endpoint(&self) -> String {
        let host = self.origin_host();
        if host.starts_with("http://") {
            host.replace("http://", "ws://") + "/cable"
        } else if host.starts_with("https://") {
            host.replace("https://", "wss://") + "/cable"
        } else {
            format!("wss://{}/cable", host)
        }
    }

    pub fn agent_v1_endpoint(&self) -> String {
        let host = self.origin_host();
        if host.starts_with("http://") || host.starts_with("https://") {
            format!("{}/agent/v1/report", host)
        } else {
            format!("https://{}/agent/v1/report", host)
        }
    }

    pub fn command(&self) -> &Commands {
        &self.args.command
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::args::{Args, Commands, CommonArgs};

    #[test]
    fn test_origin_host_with_scheme() {
        let args = Args {
            command: Commands::Run {
                reg_code: None,
                common_args: CommonArgs {
                    origin_host: Some("http://example.com".to_string()),
                },
            },
        };
        let config = Config::new(args);
        assert_eq!(config.origin_host(), "http://example.com");
    }

    #[test]
    fn test_origin_host_with_scheme_and_port() {
        let args = Args {
            command: Commands::Run {
                reg_code: None,
                common_args: CommonArgs {
                    origin_host: Some("http://example.com:3000".to_string()),
                },
            },
        };
        let config = Config::new(args);
        assert_eq!(config.origin_host(), "http://example.com:3000");
    }

    #[test]
    fn test_origin_host_without_scheme() {
        let args = Args {
            command: Commands::Run {
                reg_code: None,
                common_args: CommonArgs {
                    origin_host: Some("example.com".to_string()),
                },
            },
        };
        let config = Config::new(args);
        assert_eq!(config.origin_host(), "https://example.com");
    }

    #[test]
    fn test_origin_host_with_https_scheme() {
        let args = Args {
            command: Commands::Run {
                reg_code: None,
                common_args: CommonArgs {
                    origin_host: Some("https://example.com".to_string()),
                },
            },
        };
        let config = Config::new(args);
        assert_eq!(config.origin_host(), "https://example.com");
    }

    #[test]
    fn test_cable_endpoint_with_http_scheme() {
        let args = Args {
            command: Commands::Run {
                reg_code: None,
                common_args: CommonArgs {
                    origin_host: Some("http://example.com".to_string()),
                },
            },
        };
        let config = Config::new(args);
        assert_eq!(config.cable_endpoint(), "ws://example.com/cable");
    }

    #[test]
    fn test_cable_endpoint_with_http_scheme_and_port() {
        let args = Args {
            command: Commands::Run {
                reg_code: None,
                common_args: CommonArgs {
                    origin_host: Some("http://example.com:3000".to_string()),
                },
            },
        };
        let config = Config::new(args);
        assert_eq!(config.cable_endpoint(), "ws://example.com:3000/cable");
    }

    #[test]
    fn test_cable_endpoint_with_https_scheme() {
        let args = Args {
            command: Commands::Run {
                reg_code: None,
                common_args: CommonArgs {
                    origin_host: Some("https://example.com".to_string()),
                },
            },
        };
        let config = Config::new(args);
        assert_eq!(config.cable_endpoint(), "wss://example.com/cable");
    }

    #[test]
    fn test_cable_endpoint_without_scheme() {
        let args = Args {
            command: Commands::Run {
                reg_code: None,
                common_args: CommonArgs {
                    origin_host: Some("example.com".to_string()),
                },
            },
        };
        let config = Config::new(args);
        assert_eq!(config.cable_endpoint(), "wss://example.com/cable");
    }
}
