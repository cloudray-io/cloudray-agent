use crate::global_env_file::{env_file, write_gobal_env};
use crate::os;
use tracing::info;

pub fn install_service(reg_code: String, origin_host: Option<String>) -> anyhow::Result<()> {
    write_gobal_env(reg_code, origin_host)?;
    info!("Registration code saved to {}", env_file());

    os::wrapper::install_service()?;

    Ok(())
}
