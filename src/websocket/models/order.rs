use crate::enums::{ExecType, InstType, OrdState, OrdType, TdMode};
use crate::parser::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub inst_type: InstType,
    pub inst_id: String,
    pub ccy: String,
    pub ord_id: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub cl_ord_id: Option<String>,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub tag: Option<String>,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub px: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub sz: f64,
    pub ord_type: OrdType,
    pub side: String,
    pub pos_side: String,
    pub td_mode: TdMode,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub fill_sz: f64,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub fill_px: Option<f64>,
    pub trade_id: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub acc_fill_sz: f64,
    pub fill_time: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub fill_fee: f64,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub fill_fee_ccy: Option<String>,
    #[serde(deserialize_with = "crate::parser::deserialize_str_opt")]
    pub exec_type: Option<ExecType>,
    pub state: OrdState,
    pub avg_px: String,
    pub lever: String,
    pub tp_trigger_px: String,
    pub tp_ord_px: String,
    pub sl_trigger_px: String,
    pub sl_ord_px: String,
    pub fee_ccy: String,
    pub fee: String,
    pub rebate_ccy: String,
    pub rebate: String,
    pub pnl: String,
    pub category: String,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub u_time: DateTime<Utc>,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub c_time: DateTime<Utc>,
    pub req_id: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub amend_result: Option<i64>,
    pub code: String,
    pub msg: String,
}
