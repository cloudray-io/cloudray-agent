use crate::config::Config;
use crate::generated::pb::o2a::o2a_message::O2aPayload;
use crate::net::handshake::fetch_agent_token;
use crate::net::receive::receive_messages;
use crate::net::talk::talk;
use crate::o2a_messages::run_runlog::process_run_runlog;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

pub async fn start(config: Arc<Config>) -> anyhow::Result<()> {
    let agent_token = fetch_agent_token(&config).await?;

    loop {
        let o2a_root = talk(&config, &agent_token).await?;

        for message in o2a_root.o2a_messages {
            if let Some(payload) = message.o2a_payload {
                match payload {
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
