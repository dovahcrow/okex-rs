use super::Channel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorLiteral {
    Error,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoginLiteral {
    Login,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Message<Row = Value> {
    Event {
        event: String,
        arg: Channel,
    },
    Login {
        event: LoginLiteral,
        code: String,
        msg: String,
    },
    Error {
        event: ErrorLiteral,
        code: String,
        msg: String,
    },
    Data {
        arg: Channel,
        action: Option<Action>,
        data: Vec<Row>,
    },
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Snapshot,
    Update,
}
