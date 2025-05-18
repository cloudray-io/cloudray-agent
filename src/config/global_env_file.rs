use crate::config::config::DEFAULT_ENV_FILE_PATH;
use anyhow::anyhow;
use std::path::Path;
use std::{env, fs};
use tracing::info;

pub fn env_file() -> String {
    env::var("CLOUDRAY_ENV_FILE").unwrap_or(DEFAULT_ENV_FILE_PATH.to_string())
}

pub fn read_global_env() -> anyhow::Result<()> {
    let path_str = env_file();
    let path = Path::new(&path_str);
    if !path.exists() {
        return Ok(());
    }

    info!("Loading env file: {}", path.display());
    dotenvy::from_path(path).map_err(|e| anyhow!("Failed to read env file: {}", e))?;
    Ok(())
}

pub fn write_gobal_env(reg_code: String, origin_host: Option<String>) -> anyhow::Result<()> {
    let path_str = env_file();
    let path = Path::new(&path_str);

    let dir = path
        .parent()
        .ok_or(anyhow!("Failed to get config directory"))?;

    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| {
            anyhow!(
                "Failed to create config directory: {}\n Re-run with sudo",
                e
            )
        })?;
    }

    let mut content = format!("CLOUDRAY_REG_CODE={}\n", reg_code);

    if let Some(host) = origin_host {
        content.push_str(&format!("CLOUDRAY_ORIGIN_HOST={}\n", host));
    }

    if path.exists() {
        let existing_content =
            fs::read_to_string(path).map_err(|e| anyhow!("Failed to read env file: {}", e))?;

        content.push_str(&existing_content);
    }

    fs::write(path, content)
        .map_err(|e| anyhow!("Failed to write env file: {}\nRe-run with sudo", e))?;

    Ok(())
}
