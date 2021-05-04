use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Clone, Copy, Debug, EnumString, Deserialize, Serialize)]
pub enum ExecType {
    T,
    M,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstType {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
    Any,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TdMode {
    Cross,
    Isolated,
    Cash,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdType {
    Market,
    Limit,
    PostOnly,
    Fok,
    Ioc,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PosSide {
    Long,
    Short,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Canceled,
    Live,
    PartiallyFilled,
    Filled,
}
