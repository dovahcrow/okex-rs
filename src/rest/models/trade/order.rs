use super::super::{OrderType, PositionSide, Request, Side, TradeMode};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub inst_id: String,
    pub td_mode: TradeMode,
    pub ccy: Option<String>,
    pub tag: Option<String>,
    pub side: Side,
    pub pos_side: Option<PositionSide>,
    pub ord_type: OrderType,
    pub sz: String,
    pub px: Option<String>,
}

impl OrderRequest {
    pub fn market(inst_id: &str, td_mode: TradeMode, side: Side, qty: f64) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode,
            ccy: None,
            tag: None,
            side,
            pos_side: None,
            ord_type: OrderType::Market,
            sz: qty.to_string(),
            px: None,
        }
    }

    pub fn limit(inst_id: &str, td_mode: TradeMode, side: Side, price: f64, qty: f64) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode,
            ccy: None,
            tag: None,
            side,
            pos_side: None,
            ord_type: OrderType::Limit,
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
    pub cl_ord_id: String,
    pub tag: String,
    pub s_code: String,
    pub s_msg: String,
}

impl Request for OrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/trade/order";
    const HAS_PAYLOAD: bool = true;
    type Response = [OrderResponse; 1];
}
