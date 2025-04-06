#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::env;
use std::fs;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::process::Command;
use sysinfo::System;

#[allow(dead_code)]
#[derive(Debug)]
pub enum DeviceUidType {
    MachineId,
    ProductUuid,
    CpuSerial,
    MacAddress,
    Hostname,
}

#[allow(dead_code)]
pub fn device_uid_as_string() -> Option<String> {
    let uid = device_uid();

    uid.map(|(uid_type, value)| {
        let type_str = match uid_type {
            DeviceUidType::MachineId => "machine-id",
            DeviceUidType::ProductUuid => "product-uuid",
            DeviceUidType::CpuSerial => "cpu-serial",
            DeviceUidType::MacAddress => "mac-address",
            DeviceUidType::Hostname => "hostname",
        };
        format!("{}:{}", type_str, value)
    })
}

#[allow(dead_code)]
pub fn device_uid() -> Option<(DeviceUidType, String)> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(id) = fs::read_to_string("/etc/machine-id") {
            let id = id.trim();
            if !id.is_empty() {
                return Some((DeviceUidType::MachineId, id.to_string()));
            }
        }

        if let Ok(uuid) = fs::read_to_string("/sys/class/dmi/id/product_uuid") {
            let uuid = uuid.trim();
            if !uuid.is_empty() {
                return Some((DeviceUidType::ProductUuid, uuid.to_string()));
            }
        }

        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            for line in cpuinfo.lines() {
                if line.to_lowercase().contains("serial") {
                    if let Some(serial) = line.split(':').nth(1) {
                        let serial = serial.trim();
                        if !serial.is_empty() {
                            return Some((DeviceUidType::CpuSerial, serial.to_string()));
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("ioreg")
            .args(&["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("IOPlatformUUID") {
                    if let Some(uuid) = line.split('=').nth(1) {
                        let uuid = uuid.replace("\"", "").trim().to_string();
                        if !uuid.is_empty() {
                            return Some((DeviceUidType::ProductUuid, uuid));
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = Command::new("wmic")
            .args(&["csproduct", "get", "uuid"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().skip(1) {
                let id = line.trim();
                if !id.is_empty() {
                    return Some((DeviceUidType::ProductUuid, id.to_string()));
                }
            }
        }
    }

    if let Ok(Some(mac)) = mac_address::get_mac_address() {
        return Some((DeviceUidType::MacAddress, mac.to_string()));
    }

    if let Some(hostname) = System::host_name() {
        return Some((DeviceUidType::Hostname, hostname));
    }

    None
}
