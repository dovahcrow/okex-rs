use crate::enums::InstType;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub inst_type: InstType,
    pub inst_id: String,
    pub last: String,
    pub last_sz: String,
    pub ask_px: String,
    pub ask_sz: String,
    pub bid_px: String,
    pub bid_sz: String,
    pub open24h: String,
    pub high24h: String,
    pub low24h: String,
    pub vol_ccy24h: String,
    pub vol24h: String,
    pub sod_utc0: String,
    pub sod_utc8: String,
    pub ts: String,
}
