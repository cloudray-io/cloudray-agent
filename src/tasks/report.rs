use crate::config::{
    get_agent_token, get_send_report_at, set_send_report_at, set_send_report_at_now, REPORT_CHECK_EVERY,
    REPORT_EVERY,
};
use crate::generated::pb::o2a::o2a_message::O2aPayload;
use crate::net::handshake::handshake;
use crate::net::talk::talk;
use crate::o2a_messages::auth_result::process_auth_result;
use crate::o2a_messages::runlog_run::process_run_runlog;
use std::time::Instant;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

pub async fn run_report_task() -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(report())
}

async fn report() -> anyhow::Result<()> {
    loop {
        if get_send_report_at().await <= Instant::now() {
            match send_report().await {
                Ok(_) => {}
                Err(err) => {
                    error!("Error when reporting: {:?}", err);
                }
            }

            set_send_report_at(Instant::now() + REPORT_EVERY).await;
        }
        debug!("Sleeping for {:?}", REPORT_CHECK_EVERY);
        tokio::time::sleep(REPORT_CHECK_EVERY).await
    }
}

async fn send_report() -> anyhow::Result<()> {
    let agent_token = get_agent_token().await;

    let o2a_root = if let Some(agent_token) = agent_token {
        talk(&agent_token).await?
    } else {
        let result = handshake().await?;
        set_send_report_at_now().await;
        result
    };

    for message in o2a_root.messages {
        if let Some(payload) = message.o2a_payload {
            match payload {
                O2aPayload::AuthResult(payload) => {
                    info!("Received AuthResult payload: {:?}", payload.agent_token);
                    process_auth_result(payload).await?;
                }

                O2aPayload::RunlogRun(payload) => {
                    info!("Received RunScript payload: {:?}", payload);
                    process_run_runlog(payload).await;
                }
                O2aPayload::Error(payload) => {
                    error!("Received Error payload: {:?}", payload);
                }
            }
        }
    }

    Ok(())
}
