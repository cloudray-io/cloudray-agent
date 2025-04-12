use crate::config::{AGENT_TOKEN, REPORT_EVERY};
use crate::generated::pb::o2a::o2a_message::O2aPayload;
use crate::net::handshake::handshake;
use crate::net::talk::talk;
use crate::o2a_messages::process_auth_result::process_auth_result;
use crate::o2a_messages::run_runlog::process_run_runlog;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

pub async fn run_report_task() -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(report())
}

async fn report() -> anyhow::Result<()> {
    loop {
        let agent_token = AGENT_TOKEN.read().await.clone().to_owned();

        let o2a_root = if let Some(agent_token) = agent_token {
            talk(&agent_token).await?
        } else {
            handshake().await?
        };

        for message in o2a_root.messages {
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

        debug!("Sleeping for 5 seconds");
        tokio::time::sleep(REPORT_EVERY).await
    }
}
