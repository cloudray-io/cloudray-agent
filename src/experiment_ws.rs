use crate::cable::request::Request;
use crate::cable::response::Response;
use crate::config::Config;
use anyhow::anyhow;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};

#[allow(dead_code)]
pub async fn start(config: Arc<Config>) -> anyhow::Result<()> {
    let cable_url = config.cable_endpoint();

    info!("Connecting to {}", cable_url);
    let socket = connect_async(cable_url).await?;
    let (socket, _) = socket;
    let (mut writer, mut reader) = socket.split();

    let params = serde_json::json!({
        "channel": "ServerTimeChannel",
    });

    let subscribe_payload = Request::new_subscribe(params)?.to_string()?;
    info!("Sending subscribe request");
    info!("{:?}", subscribe_payload);
    writer.send(Message::Text(subscribe_payload.into())).await?;

    loop {
        let msg = reader.next().await;
        if let Some(Ok(msg)) = msg {
            let response = Response::new(msg)?;
            println!("type: {:?}", response.payload.r#type);
            println!("data: {:?}", response.payload.message);
        } else {
            error!("Error reading from socket");
            return Err(anyhow!("Error reading from socket".to_string(),));
        }
    }
}
