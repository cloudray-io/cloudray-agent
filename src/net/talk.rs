use crate::config::Config;
use crate::generated::pb::o2a::O2aRoot;
use crate::message_queue::MessageQueue;
use crate::net::receive::receive_messages;
use crate::types::AgentToken;
use std::sync::Arc;

pub async fn talk(config: &Arc<Config>, agent_token: &AgentToken) -> anyhow::Result<O2aRoot> {
    let url = config.agent_v1_talk_endpoint();

    let mut payloads = Vec::new();

    loop {
        let message = MessageQueue::pop_front().await;
        if message.is_none() {
            break;
        }
        payloads.push(message.unwrap());
    }

    receive_messages(url, Some(agent_token), payloads).await
}
