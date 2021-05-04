use super::super::Request;
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub inst_id: String,
    pub ord_id: Option<String>,
}

impl CancelOrderRequest {
    pub fn with_ord_id(inst_id: &str, ord_id: &str) -> Self {
        Self {
            inst_id: inst_id.to_string(),
            ord_id: Some(ord_id.into()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub ord_id: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub cl_ord_id: Option<String>,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub s_code: u64,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub s_msg: Option<String>,
}

impl Request for CancelOrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/trade/cancel-order";
    const HAS_PAYLOAD: bool = true;
    type Response = [CancelOrderResponse; 1];
}
