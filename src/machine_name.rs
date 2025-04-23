use sysinfo::System;

pub fn machine_name() -> String {
    if let Some(hostname) = System::host_name() {
        return hostname;
    }

    if let Some(os_version_long) = System::long_os_version() {
        return os_version_long;
    }

    format!("{} {}", std::env::consts::FAMILY, std::env::consts::ARCH)
        .trim()
        .to_string()
}
