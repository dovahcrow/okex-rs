use super::super::Request;
use crate::enums::{InstType, OrdState, OrdType, PosSide, Side, TdMode};
use crate::parser::ts_milliseconds;
use chrono::{DateTime, Utc};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub inst_id: String,
    pub td_mode: TdMode,
    pub ccy: Option<String>,
    pub tag: Option<String>,
    pub side: Side,
    pub pos_side: Option<PosSide>,
    pub ord_type: OrdType,
    pub sz: String,
    pub px: Option<String>,
}

impl PlaceOrderRequest {
    pub fn market(inst_id: &str, td_mode: TdMode, side: Side, qty: f64) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode,
            ccy: None,
            tag: None,
            side,
            pos_side: None,
            ord_type: OrdType::Market,
            sz: qty.to_string(),
            px: None,
        }
    }

    pub fn limit(inst_id: &str, td_mode: TdMode, side: Side, price: f64, qty: f64) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode,
            ccy: None,
            tag: None,
            side,
            pos_side: None,
            ord_type: OrdType::Limit,
            sz: qty.to_string(),
            px: Some(price.to_string()),
        }
    }

    pub fn set_ccy(&mut self, ccy: &str) -> &mut Self {
        self.ccy = Some(ccy.to_string());
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub ord_id: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub cl_ord_id: Option<String>,
    pub tag: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub s_code: u64,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub s_msg: Option<String>,
}

impl Request for PlaceOrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/trade/order";
    const HAS_PAYLOAD: bool = true;
    type Response = [PlaceOrderResponse; 1];
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailsRequest {
    pub inst_id: String,
    pub ord_id: Option<String>,
    pub cl_ord_id: Option<String>,
}

impl OrderDetailsRequest {
    pub fn ord_id(inst_id: &str, ord_id: &str) -> Self {
        Self {
            inst_id: inst_id.into(),
            ord_id: Some(ord_id.into()),
            cl_ord_id: None,
        }
    }

    pub fn cl_ord_id(inst_id: &str, cl_ord_id: &str) -> Self {
        Self {
            inst_id: inst_id.into(),
            ord_id: None,
            cl_ord_id: Some(cl_ord_id.into()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailsResponse {
    pub inst_type: InstType,
    pub inst_id: String,
    pub ccy: String,
    pub ord_id: String,
    pub cl_ord_id: String,
    pub tag: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub px: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub sz: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub pnl: f64,
    pub ord_type: OrdType,
    pub side: Side,
    pub pos_side: PosSide,
    pub td_mode: TdMode,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub acc_fill_sz: f64,
    pub fill_px: String,
    pub trade_id: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub fill_sz: f64,
    pub fill_time: String,
    pub state: OrdState,
    pub avg_px: String,
    pub lever: String,
    pub tp_trigger_px: String,
    pub tp_ord_px: String,
    pub sl_trigger_px: String,
    pub sl_ord_px: String,
    pub fee_ccy: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub fee: f64,
    pub rebate_ccy: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub rebate: f64,
    pub category: String,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub u_time: DateTime<Utc>,
    #[serde(deserialize_with = "ts_milliseconds")]
    pub c_time: DateTime<Utc>,
}

impl Request for OrderDetailsRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/trade/order";
    const HAS_PAYLOAD: bool = true;
    type Response = [OrderDetailsResponse; 1];
}
