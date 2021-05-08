use crate::enums::InstType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "channel")]
pub enum Channel {
    #[serde(rename = "books-l2-tbt")]
    BooksL2Tbt {
        #[serde(rename = "instId")]
        inst_id: String,
    },
    #[serde(rename = "orders")]
    Orders {
        #[serde(rename = "instType")]
        inst_type: InstType,
        uly: Option<String>,
        #[serde(rename = "instId")]
        inst_id: Option<String>,
    },
    #[serde(rename = "tickers")]
    Tickers {
        #[serde(rename = "instId")]
        inst_id: String,
    },
}
