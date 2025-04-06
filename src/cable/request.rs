use serde::de::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestPayload {
    pub command: CommandType,
    pub identifier: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum CommandType {
    #[serde(rename = "subscribe")]
    Subscribe,
    #[serde(rename = "message")]
    Message,

    #[serde(untagged)]
    Other(String),
}

#[allow(dead_code)]
pub struct Request {
    pub payload: RequestPayload,
}

#[allow(dead_code)]
impl Request {
    pub fn new_subscribe<T: Serialize>(identifier: T) -> Result<Self, serde_json::Error> {
        let value = serde_json::to_value(&identifier)?;

        if value.is_object() && value.get("channel").is_none() {
            return Err(serde_json::Error::custom(
                "identifier must contain 'channel' key",
            ));
        }

        let identifier = serde_json::to_string(&value)?;

        Ok(Self {
            payload: RequestPayload {
                command: CommandType::Subscribe,
                identifier: Some(identifier),
            },
        })
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.payload)
    }
}
