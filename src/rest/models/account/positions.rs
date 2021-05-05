use super::super::Request;
use crate::enums::{InstType, MgnMode};
use crate::parser::ts_milliseconds;
use chrono::{DateTime, Utc};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PositionsRequest {
    pub inst_type: Option<InstType>,
    pub inst_id: Option<String>,
    pub pos_id: Option<String>,
}

impl PositionsRequest {
    pub fn inst_type(inst_type: InstType) -> Self {
        Self {
            inst_type: Some(inst_type),
            inst_id: None,
            pos_id: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsResponse {
    pub inst_id: String,
    pub inst_type: InstType,
    pub mgn_mode: MgnMode,
    pub pos_id: String,
    pub pos_side: String,
    pub pos: String,
    pub ccy: String,
    pub pos_ccy: String,
    pub avail_pos: String,
    pub avg_px: String,
    pub upl: String,
    pub upl_ratio: String,
    pub lever: String,
    pub liq_px: String,
    pub imr: String,
    pub margin: String,
    pub mgn_ratio: String,
    pub mmr: String,
    pub liab: String,
    pub liab_ccy: String,
    pub interest: String,
    pub trade_id: String,
    pub opt_val: String,
    pub adl: String,
    pub last: String,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub c_time: DateTime<Utc>,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub u_time: DateTime<Utc>,
}

impl Request for PositionsRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/account/positions";
    const HAS_PAYLOAD: bool = false;
    type Response = Vec<PositionsResponse>;
}
