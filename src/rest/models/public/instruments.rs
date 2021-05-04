use super::super::Request;
use crate::enums::{Alias, CtType, InstType, InstrumentState, OptType};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsRequest {
    pub inst_type: InstType,
    pub uly: Option<String>,
    pub inst_id: Option<String>,
}

impl InstrumentsRequest {
    pub fn inst_type(inst_type: InstType) -> Self {
        Self {
            inst_type,
            uly: None,
            inst_id: None,
        }
    }

    pub fn inst_id(inst_type: InstType, inst_id: &str) -> Self {
        Self {
            inst_type,
            uly: None,
            inst_id: Some(inst_id.into()),
        }
    }

    pub fn uly(inst_type: InstType, uly: &str) -> Self {
        Self {
            inst_type,
            uly: Some(uly.into()),
            inst_id: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsResponse {
    pub inst_type: InstType,
    pub inst_id: String,
    pub uly: String,
    pub category: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub base_ccy: Option<String>,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub quote_ccy: Option<String>,
    pub settle_ccy: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub ct_val: Option<f64>,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub ct_mult: Option<f64>,
    pub ct_val_ccy: String,
    #[serde(deserialize_with = "crate::parser::deserialize_str_opt")]
    pub opt_type: Option<OptType>,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub stk: Option<f64>,
    pub list_time: String,
    pub exp_time: String,
    #[serde(deserialize_with = "crate::parser::from_str_opt")]
    pub lever: Option<u64>,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub tick_sz: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub lot_sz: f64,
    #[serde(deserialize_with = "crate::parser::from_str")]
    pub min_sz: f64,
    #[serde(deserialize_with = "crate::parser::deserialize_str_opt")]
    pub ct_type: Option<CtType>,
    #[serde(deserialize_with = "crate::parser::deserialize_str_opt")]
    pub alias: Option<Alias>,
    pub state: InstrumentState,
}

impl Request for InstrumentsRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = false;
    const ENDPOINT: &'static str = "/api/v5/public/instruments";
    const HAS_PAYLOAD: bool = true;
    type Response = Vec<InstrumentsResponse>;
}
