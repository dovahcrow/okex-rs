use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Clone)]
pub enum OkExError {
    #[error("Cannot deserialize response from {0}")]
    CannotDeserializeResponse(String),
    #[error("No Api key set for private api")]
    NoApiKeySet,
    #[error("{name} error message from BitMEX server: {message}")]
    RemoteError { message: String, name: String },
    #[error("Websocket closed")]
    WebsocketClosed,
    #[error("Unexpected websocket binary content {0:?}")]
    UnexpectedWebsocketBinaryContent(Vec<u8>),
    #[error("Cannot parse topic {0:?}")]
    ParseTopicError(String),
    #[error("Error from websocket. {status}: {error}")]
    WebsocketError { status: i64, error: String },
}
