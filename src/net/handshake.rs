use crate::config::CONFIG;
use crate::device_uid::device_uid_as_string;
use crate::generated::pb::a2o::a2o_message::A2oPayload;
use crate::generated::pb::a2o::AuthRequestToken;
use crate::generated::pb::o2a::O2aRoot;
use crate::machine_name::machine_name;
use crate::net::receive::receive_messages;
use crate::version::agent_version_as_pb;
use anyhow::anyhow;
use sysinfo::System;

pub async fn handshake() -> anyhow::Result<O2aRoot> {
    let url = CONFIG.agent_v1_endpoint();
    let mut payloads = Vec::new();

    let payload = AuthRequestToken {
        reg_code: CONFIG
            .reg_code()
            .ok_or(anyhow!("Registration code is not set."))?,
        machine_uid: device_uid_as_string().unwrap_or_default(),
        machine_name: machine_name(),
        agent_version: Some(agent_version_as_pb()),

        os_arch: std::env::consts::ARCH.to_string(),
        os_family: std::env::consts::FAMILY.to_string(),
        os_name: std::env::consts::OS.to_string(),
        os_distro: System::distribution_id(),
        os_version: System::os_version().unwrap_or_default(),
        os_version_long: System::long_os_version().unwrap_or_default(),
    };

    payloads.push(A2oPayload::AuthRequestToken(payload));
    receive_messages(url, None, payloads).await
}
