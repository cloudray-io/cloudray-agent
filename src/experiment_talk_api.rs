use crate::config::AGENT_TOKEN;
use crate::generated::pb::o2a::o2a_message::O2aPayload;
use crate::net::handshake::handshake;
use crate::net::talk::talk;
use crate::o2a_messages::process_auth_result::process_auth_result;
use crate::o2a_messages::run_runlog::process_run_runlog;
use std::time::Duration;
use tracing::{error, info};

pub async fn start() -> anyhow::Result<()> {
    loop {
        let agent_token = AGENT_TOKEN.read().await;

        let o2a_root = if let Some(agent_token) = agent_token.as_ref() {
            talk(agent_token).await?
        } else {
            handshake().await?
        };

        for message in o2a_root.o2a_messages {
            if let Some(payload) = message.o2a_payload {
                match payload {
                    O2aPayload::AuthResult(payload) => {
                        info!("Received AuthResult payload: {:?}", payload.agent_token);
                        process_auth_result(payload).await?;
                    }

                    O2aPayload::RunRunlog(payload) => {
                        info!("Received RunScript payload: {:?}", payload);
                        process_run_runlog(payload).await?;
                    }
                    _ => {
                        error!("Received unknown payload type: {:?}", payload);
                    }
                }
            }
        }

        println!("Sleeping for 5 seconds");
        tokio::time::sleep(Duration::from_secs(5)).await
    }
}
