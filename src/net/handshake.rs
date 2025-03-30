use crate::config::Config;
use crate::generated::pb::a2o::a2o_message::A2oPayload;
use crate::generated::pb::a2o::AuthRequestToken;
use crate::generated::pb::o2a::o2a_message::O2aPayload;
use crate::generated::pb::o2a::O2aRoot;
use crate::net::receive::receive_messages;
use crate::types::AgentToken;
use anyhow::anyhow;
use std::sync::Arc;

pub async fn handshake(config: &Arc<Config>) -> anyhow::Result<O2aRoot> {
    let url = config.agent_v1_handshake_endpoint();
    let mut payloads = Vec::new();

    let payload = AuthRequestToken {
        reg_code: config.reg_code().clone(),
    };

    payloads.push(A2oPayload::AuthRequestToken(payload));
    receive_messages(url, None, payloads).await
}

pub async fn fetch_agent_token(config: &Arc<Config>) -> anyhow::Result<AgentToken> {
    let handshake = handshake(config).await?;
    let auth_result = handshake
        .o2a_messages
        .into_iter()
        .find_map(|m| match m.o2a_payload {
            Some(O2aPayload::AuthResult(auth_result)) => Some(auth_result),
            _ => None,
        })
        .ok_or_else(|| anyhow!("Handshake did not return an AuthResult"))?;
    Ok(AgentToken(auth_result.agent_token))
}
