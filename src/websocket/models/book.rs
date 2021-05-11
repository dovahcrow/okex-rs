use crate::parser::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct LevelInfo(
    #[serde(deserialize_with = "crate::parser::from_str")] pub f64,
    #[serde(deserialize_with = "crate::parser::from_str")] pub f64,
    #[serde(deserialize_with = "crate::parser::from_str")] pub f64,
    #[serde(deserialize_with = "crate::parser::from_str")] pub f64,
);

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub asks: Vec<LevelInfo>,
    pub bids: Vec<LevelInfo>,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub ts: DateTime<Utc>,
    pub checksum: i64,
}
