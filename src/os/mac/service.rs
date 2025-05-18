use crate::config::get_executable_path;
use anyhow::anyhow;
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::info;

pub fn create_launchd_plist() -> anyhow::Result<()> {
    let executable_path = get_executable_path()?;
    let executable_path_str = executable_path
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert executable path to string"))?;

    let launch_dir = Path::new("/Library/LaunchDaemons");

    let plist_path = launch_dir.join("io.cloudray.agent.plist");
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>io.cloudray.agent</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>run</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardErrorPath</key>
    <string>/tmp/cloudray-agent.err</string>
    <key>StandardOutPath</key>
    <string>/tmp/cloudray-agent.log</string>
</dict>
</plist>
"#,
        executable_path_str
    );

    fs::write(&plist_path, plist_content)?;
    info!("Created launchd plist file at {}", plist_path.display());

    Ok(())
}

pub fn load_or_reload_service() -> anyhow::Result<()> {
    let plist_path = Path::new("/Library/LaunchDaemons/io.cloudray.agent.plist");
    let plist_path_str = plist_path
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert plist path to string"))?;

    let output = Command::new("launchctl")
        .args(["list", "io.cloudray.agent"])
        .output()
        .map_err(|e| anyhow!("Failed to check service status: {}", e))?;

    if !output.status.success() {
        // Service not loaded, load it
        Command::new("launchctl")
            .args(["load", "-w", plist_path_str])
            .output()
            .map_err(|e| anyhow!("Failed to load service: {}", e))?;
        info!("Loaded cloudray-agent service");
    } else {
        // Service already loaded, unload and reload
        Command::new("launchctl")
            .args(["unload", plist_path_str])
            .output()
            .map_err(|e| anyhow!("Failed to unload service: {}", e))?;
        Command::new("launchctl")
            .args(["load", "-w", plist_path_str])
            .output()
            .map_err(|e| anyhow!("Failed to reload service: {}", e))?;
        info!("Reloaded cloudray-agent service");
    }

    Ok(())
}

pub fn install_macos_service() -> anyhow::Result<()> {
    create_launchd_plist()?;
    load_or_reload_service()?;

    Ok(())
}
