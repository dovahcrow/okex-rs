use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
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
        inst_type: String,
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

// MARGIN
// SWAP
// FUTURES
// OPTION
