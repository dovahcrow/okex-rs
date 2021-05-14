use crate::enums::InstType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "channel")]
pub enum Channel {
    #[serde(rename = "books")]
    Books {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "books5")]
    Books5 {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "books50-l2-tbt")]
    Books50L2Tbt {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "books-l2-tbt")]
    BooksL2Tbt {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "instruments")]
    Instruments {
        #[serde(rename = "instType")]
        inst_type: InstType,
    },
    #[serde(rename = "orders")]
    Orders {
        #[serde(rename = "instType")]
        inst_type: InstType,
        uly: Option<String>,
        #[serde(rename = "instId")]
        inst_id: Option<String>,
    },
    #[serde(rename = "price-limit")]
    PriceLimit {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "tickers")]
    Tickers {
        #[serde(rename = "instId")]
        inst_id: String,
    },
}

impl Channel {
    pub fn books(inst_id: &str) -> Self {
        Self::Books {
            inst_id: inst_id.into(),
        }
    }
    pub fn books5(inst_id: &str) -> Self {
        Self::Books5 {
            inst_id: inst_id.into(),
        }
    }

    pub fn books50_l2_tbt(inst_id: &str) -> Self {
        Self::Books50L2Tbt {
            inst_id: inst_id.into(),
        }
    }

    pub fn books_l2_tbt(inst_id: &str) -> Self {
        Self::BooksL2Tbt {
            inst_id: inst_id.into(),
        }
    }

    pub fn instruments(inst_type: InstType) -> Self {
        Self::Instruments { inst_type }
    }

    pub fn tickers(inst_id: &str) -> Self {
        Self::Tickers {
            inst_id: inst_id.into(),
        }
    }

    pub fn price_limit(inst_id: &str) -> Self {
        Self::PriceLimit {
            inst_id: inst_id.into(),
        }
    }
}
