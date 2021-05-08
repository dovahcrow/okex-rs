use lazy_static::lazy_static;
use std::env::var;

// dotenv is a must run in every test otherwise the url will be mis-loaded
lazy_static! {
    pub static ref PUB_WS_URL: &'static str = {
        if var("OKEX_AWS").unwrap_or_else(|_| "0".to_string()) == "0" {
            "wss://ws.okex.com:8443/ws/v5/public"
        } else {
            "wss://wsaws.okex.com:8443/ws/v5/public"
        }
    };
    pub static ref PRIV_WS_URL: &'static str = {
        if var("OKEX_AWS").unwrap_or_else(|_| "0".to_string()) == "0" {
            "wss://ws.okex.com:8443/ws/v5/private"
        } else {
            "wss://wsaws.okex.com:8443/ws/v5/private"
        }
    };
    pub static ref REST_URL: &'static str = {
        if var("OKEX_AWS").unwrap_or_else(|_| "0".to_string()) == "0" {
            "https://www.okex.com"
        } else {
            "https://aws.okex.com"
        }
    };
    pub static ref IS_AWS: bool = var("OKEX_AWS").unwrap_or_else(|_| "0".to_string()) != "0";
}
