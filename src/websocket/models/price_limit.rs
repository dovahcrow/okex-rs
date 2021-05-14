use crate::parser::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimit {
    inst_id: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    buy_lmt: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    sell_lmt: f64,
    #[serde(deserialize_with = "ts_milliseconds")]
    ts: DateTime<Utc>,
}
