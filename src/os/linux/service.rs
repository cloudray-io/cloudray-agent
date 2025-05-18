use crate::config::get_executable_path;
use anyhow::anyhow;
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::info;

pub fn create_systemd_service() -> anyhow::Result<()> {
    let executable_path = get_executable_path()?;
    let executable_path_str = executable_path
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert executable path to string"))?;

    let service_dir = Path::new("/etc/systemd/system");

    let service_path = service_dir.join("cloudray-agent.service");
    let service_content = format!(
        r#"[Unit]
Description=CloudRay Agent
After=network.target

[Service]
ExecStart={} run
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
"#,
        executable_path_str
    );

    fs::write(&service_path, service_content)?;
    info!("Created systemd service file at {}", service_path.display());

    Ok(())
}

pub fn reload_systemd() -> anyhow::Result<()> {
    Command::new("systemctl")
        .args(["daemon-reload"])
        .output()
        .map_err(|e| anyhow!("Failed to reload systemd: {}", e))?;

    Ok(())
}

pub fn enable_service() -> anyhow::Result<()> {
    Command::new("systemctl")
        .args(["enable", "cloudray-agent.service"])
        .output()
        .map_err(|e| anyhow!("Failed to enable service: {}", e))?;

    Ok(())
}

pub fn start_or_restart_service() -> anyhow::Result<()> {
    let output = Command::new("systemctl")
        .args(["is-active", "cloudray-agent.service"])
        .output()
        .map_err(|e| anyhow!("Failed to check service status: {}", e))?;

    if String::from_utf8_lossy(&output.stdout).trim() == "active" {
        Command::new("systemctl")
            .args(["restart", "cloudray-agent.service"])
            .output()
            .map_err(|e| anyhow!("Failed to restart service: {}", e))?;
        info!("Restarted cloudray-agent service");
    } else {
        Command::new("systemctl")
            .args(["start", "cloudray-agent.service"])
            .output()
            .map_err(|e| anyhow!("Failed to start service: {}", e))?;
        info!("Started cloudray-agent service");
    }

    Ok(())
}

pub fn install_linux_service() -> anyhow::Result<()> {
    create_systemd_service()?;
    reload_systemd()?;
    enable_service()?;
    start_or_restart_service()?;

    Ok(())
}
