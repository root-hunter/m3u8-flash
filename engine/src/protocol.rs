use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    Ping,
    Pong,
    Text { text: String },
    Command(CommandData),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "command_type", content = "payload")]
pub enum CommandData {
    StartExport { uid: String, url: String },
}