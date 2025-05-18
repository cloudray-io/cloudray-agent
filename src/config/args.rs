use clap::{Args as ClapArgs, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "Learn more at https://github.com/cloudray-io/cloudray-agent", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install the agent as a service
    InstallService {
        /// Registration code. You can find it here https://app.cloudray.io
        #[arg(long, env = "CLOUDRAY_REG_CODE")]
        reg_code: String,

        #[clap(flatten)]
        common_args: CommonArgs,
    },

    /// Run the agent in foreground
    Run {
        /// Registration code. You can find it here https://app.cloudray.io
        #[arg(long, env = "CLOUDRAY_REG_CODE")]
        reg_code: Option<String>,

        #[clap(flatten)]
        common_args: CommonArgs,
    },
}

#[derive(Debug, ClapArgs)]
pub struct CommonArgs {
    /// Origin host (hidden option for development)
    #[arg(long, hide = true, env = "CLOUDRAY_ORIGIN_HOST")]
    pub origin_host: Option<String>,
}
