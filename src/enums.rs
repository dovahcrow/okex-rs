use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Alias {
    ThisWeek,
    NextWeek,
    Quarter,
    NextQuarter,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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
pub enum MgnMode {
    Cross,
    Isolated,
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
    Net,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdState {
    Canceled,
    Live,
    PartiallyFilled,
    Filled,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OptType {
    C,
    P,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CtType {
    Linear,
    Inverse,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstrumentState {
    Live,
    Suspend,
    Preopen,
    Settlement,
}
