use crate::args::Args;

const ORIGIN_HOST: &str = "api.cloudray.io";
const TEST_AGENT_TOKEN: &str = "test";

pub struct Config {
    args: Args,
}

impl Config {
    pub fn new(args: Args) -> Self {
        Self {
            args,
        }
    }

    pub fn reg_code(&self) -> &String {
        &self.args.reg_code
    }

    pub fn cable_endpoint(&self) -> String {
        let default_host = &ORIGIN_HOST.to_string();
        let host = self.args.origin_host.as_ref().unwrap_or_else(|| default_host);
        let scheme = if host.starts_with("localhost") { "ws" } else { "wss" };
        format!("{}://{}/cable?agent_token={}", scheme, host, TEST_AGENT_TOKEN)
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
