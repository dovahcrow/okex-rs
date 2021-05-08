mod channel;
mod command;
mod message;
pub mod models;

pub use self::channel::Channel;
pub use self::command::Command;
pub use self::message::{Action, Message};
use crate::consts::{PRIV_WS_URL, PUB_WS_URL};
use crate::credential::Credential;
use crate::error::OkExError;
use anyhow::Error;
use fehler::{throw, throws};
use futures::sink::Sink;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use log::trace;
pub use serde_json::Value;
use serde_json::{from_str, to_string};
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message as WSMessage;
use url::Url;

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct OkExWebsocket {
    credential: Option<Credential>,
    inner: WSStream,
}

impl OkExWebsocket {
    #[throws(OkExError)]
    pub async fn new() -> Self {
        Self::new_impl(false).await?
    }

    #[throws(OkExError)]
    pub async fn with_credential(api_key: &str, api_secret: &str, passphrase: &str) -> Self {
        let mut c = Self::new_impl(true).await?;
        c.credential = Some(Credential::new(api_key, api_secret, passphrase));
        c
    }

    #[throws(OkExError)]
    async fn new_impl(private: bool) -> Self {
        let url = if private { PRIV_WS_URL } else { PUB_WS_URL };
        let (stream, _) = connect_async(Url::parse(&url).unwrap()).await?;
        Self {
            credential: None,
            inner: stream,
        }
    }

    #[throws(OkExError)]
    fn get_credential(&self) -> &Credential {
        match self.credential.as_ref() {
            None => throw!(OkExError::NoApiKeySet),
            Some(c) => c,
        }
    }
}

impl Sink<Command> for OkExWebsocket {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(mut self: Pin<&mut Self>, item: Command) -> Result<(), Self::Error> {
        let command = match &item {
            &Command::Ping => "ping".to_string(),
            command => to_string(command)?,
        };
        trace!("Sending '{}' through websocket", command);
        let inner = Pin::new(&mut self.inner);
        Ok(inner.start_send(WSMessage::Text(command))?)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_close(cx).map_err(|e| e.into())
    }
}

impl Stream for OkExWebsocket {
    type Item = Result<Message, OkExError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let inner = Pin::new(&mut self.inner);
        let poll = inner.poll_next(cx);
        match poll {
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(Some(Ok(m))) => match parse_message(m) {
                Ok(m) => Poll::Ready(Some(Ok(m))),
                Err(e) => Poll::Ready(Some(Err(e))),
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[throws(OkExError)]
fn parse_message(msg: WSMessage) -> Message {
    match msg {
        WSMessage::Text(message) => match message.as_str() {
            "pong" => Message::Pong,
            others => match from_str(others) {
                Ok(r) => r,
                Err(_) => unreachable!("Cannot deserialize message from OkEx: '{}'", others),
            },
        },
        WSMessage::Close(_) => throw!(OkExError::WebsocketClosed),
        WSMessage::Binary(_) => throw!(OkExError::UnexpectedWebsocketBinaryMessage),
        WSMessage::Ping(_) => throw!(OkExError::UnexpectedWebsocketPingMessage),
        WSMessage::Pong(_) => throw!(OkExError::UnexpectedWebsocketPongMessage),
    }
}
