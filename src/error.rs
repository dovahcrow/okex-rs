use thiserror::Error;

pub type Result<T> = std::result::Result<T, OkExError>;
#[derive(Debug, Error)]
pub enum OkExError {
    #[error("Cannot deserialize response from {0}")]
    CannotDeserializeResponse(String),

    #[error("No Api key set for private api")]
    NoApiKeySet,

    #[error("Unexpected websocket binary message")]
    UnexpectedWebsocketBinaryMessage,

    #[error("Unexpected websocket ping message")]
    UnexpectedWebsocketPingMessage,

    #[error("Unexpected websocket pong message")]
    UnexpectedWebsocketPongMessage,

    #[error("Websocket closed")]
    WebsocketClosed,

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    UrlEncoding(#[from] serde_urlencoded::ser::Error),

    #[error(transparent)]
    JsonParse(#[from] serde_json::Error),

    #[error(transparent)]
    HttpRequest(#[from] reqwest::Error),

    #[error(transparent)]
    Websocket(#[from] tungstenite::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
