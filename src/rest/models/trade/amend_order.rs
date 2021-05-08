use super::super::Request;
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    pub inst_id: String,
    pub cxl_on_fail: bool,
    pub ord_id: Option<String>,
    pub new_sz: Option<String>,
    pub new_px: Option<String>,
}

impl AmendOrderRequest {
    pub fn new_qty(inst_id: &str, ord_id: &str, qty: f64) -> Self {
        Self {
            inst_id: inst_id.to_string(),
            cxl_on_fail: false,
            ord_id: Some(ord_id.into()),
            new_sz: Some(qty.to_string()),
            new_px: None,
        }
    }
    pub fn new_price(inst_id: &str, ord_id: &str, price: f64) -> Self {
        Self {
            inst_id: inst_id.to_string(),
            cxl_on_fail: false,
            ord_id: Some(ord_id.into()),
            new_sz: None,
            new_px: Some(price.to_string()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    pub ord_id: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub cl_ord_id: Option<String>,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub req_id: Option<String>,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub s_code: u64,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub s_msg: Option<String>,
}

impl Request for AmendOrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/trade/amend-order";
    const HAS_PAYLOAD: bool = true;
    type Response = [AmendOrderResponse; 1];
}
