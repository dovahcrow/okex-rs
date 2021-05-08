pub mod urls;
mod credential;
pub mod enums;
mod error;
mod parser;
pub mod rest;
pub mod websocket;

pub use error::{OkExError, Result};
