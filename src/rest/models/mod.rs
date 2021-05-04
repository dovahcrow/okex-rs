mod account;
mod public;
mod trade;

pub use account::*;
pub use public::*;
pub use trade::*;

use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Request: Serialize {
    const METHOD: Method;
    const SIGNED: bool = false;
    const ENDPOINT: &'static str;
    const HAS_PAYLOAD: bool = true;
    type Response: DeserializeOwned;

    #[inline]
    fn no_payload(&self) -> bool {
        !Self::HAS_PAYLOAD
    }
}
