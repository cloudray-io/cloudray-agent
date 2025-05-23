use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Learn more at https://github.com/cloudray-io/cloufray-agent", long_about = None)]
pub struct Args {
    #[arg(long, env = "CLOUDRAY_DAEMON", short = 'd')]
    pub daemon: bool,

    /// Registration code. You can find it here https://app.cloudray.io
    #[arg(long, env = "CLOUDRAY_REG_CODE")]
    pub reg_code: String,

    /// Authentication code
    #[arg(long, env = "CLOUDRAY_AUTH_CODE")]
    pub auth_code: Option<String>,

    #[arg(long, hide = true, env = "CLOUDRAY_ORIGIN_HOST")]
    pub origin_host: Option<String>,
}
