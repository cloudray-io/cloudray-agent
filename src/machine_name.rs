use sysinfo::System;

pub fn machine_name() -> String {
    if let Some(hostname) = System::host_name() {
        return hostname;
    }

    if let Some(os_version) = System::os_version() {
        return format!("{} {}", System::distribution_id(), os_version);
    }

    format!("{} {}", std::env::consts::FAMILY, std::env::consts::ARCH)
        .trim()
        .to_string()
}
