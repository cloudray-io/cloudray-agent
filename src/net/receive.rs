use crate::config::CONFIG;
use crate::generated::pb::a2o::a2o_message::A2oPayload;
use crate::generated::pb::a2o::{A2oMessage, A2oRoot};
use crate::generated::pb::o2a::O2aRoot;
use crate::types::AgentToken;
use prost::Message;
use std::time::Duration;
use tokio_tungstenite::tungstenite::client;
use tracing::debug;

pub async fn receive_messages(
    url: String,
    agent_token: Option<&AgentToken>,
    payloads: Vec<A2oPayload>,
) -> anyhow::Result<O2aRoot> {
    let mut payload = A2oRoot::default();
    if let Some(agent_token) = agent_token {
        payload.agent_token = agent_token.0.to_string();
    }
    let messages = payloads
        .into_iter()
        .map(|payload| A2oMessage {
            a2o_payload: Some(payload),
        })
        .collect();

    payload.messages = messages;

    debug!("Sending PB: {:?}", payload);

    let body = payload.encode_to_vec();

    let res = CONFIG
        .origin_client()
        .post(url)
        .header("Content-Type", "application/x-protobuf")
        .body(body)
        .send()
        .await?;

    let res_body = res.bytes().await?;

    let result = O2aRoot::decode(res_body.as_ref())?;
    debug!("Received PB: {:?}", result);

    Ok(result)
}
