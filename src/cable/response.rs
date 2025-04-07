use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct ResponsePayload {
    pub r#type: Option<String>,
    pub message: Option<Value>,
    pub identifier: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
enum MessageType {
    #[serde(rename = "confirm_subscription")]
    Confirmation,
    #[serde(rename = "disconnect")]
    Disconnect,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "reject_subscription")]
    Rejection,
    #[serde(rename = "welcome")]
    Welcome,

    Other(String),
}

impl TryFrom<Message> for ResponsePayload {
    type Error = anyhow::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Text(text) => {
                let payload: ResponsePayload = serde_json::from_str(&text)?;
                Ok(payload)
            }
            _ => Err(anyhow::anyhow!("Unexpected message type")),
        }
    }
}

#[allow(dead_code)]
pub struct Response {
    pub payload: ResponsePayload,
}

#[allow(dead_code)]
impl Response {
    pub fn new(message: Message) -> anyhow::Result<Self> {
        info!("Received:");
        info!("{:?}", message);
        let payload = ResponsePayload::try_from(message)?;
        Ok(Self { payload })
    }
}
