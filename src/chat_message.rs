use askama::Template;
use std::str::FromStr;

#[derive(Debug, Clone, Template)]
#[template(path = "components/chat_bubble.html")]
pub struct ChatMessage {
    pub username: String,
    pub body: String,
}

impl ChatMessage {
    pub fn new(username: &str, message: &str) -> Self {
        Self {
            username: username.into(),
            body: message.into(),
        }
    }
}

impl FromStr for ChatMessage {
    type Err = String;

    fn from_str(json_str: &str) -> Result<Self, Self::Err> {
        let request = serde_json::from_str::<MessageRequest>(json_str)
            .map_err(|e| format!("invalid json body {}", e))?;

        Ok(ChatMessage::new(&request.username, &request.body))
    }
}

#[derive(Debug, serde::Deserialize)]
struct MessageRequest {
    #[serde(rename = "HEADERS")]
    _headers: serde_json::Value,
    username: String,
    #[serde(rename = "body")]
    body: String,
}
