use super::super::Request;
use crate::enums::{OrdType, PosSide, Side, TdMode};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
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

impl OrderRequest {
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
pub struct OrderResponse {
    pub ord_id: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub cl_ord_id: Option<String>,
    pub tag: String,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub s_code: u64,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub s_msg: Option<String>,
}

impl Request for OrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/trade/order";
    const HAS_PAYLOAD: bool = true;
    type Response = [OrderResponse; 1];
}
