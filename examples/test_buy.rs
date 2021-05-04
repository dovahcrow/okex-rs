use anyhow::Error;
use fehler::throws;
use okex::rest::{
    AmendOrderRequest, BalanceRequest, CancelOrderRequest, OkExRest, OrderRequest, Side, TradeMode,
};
use std::env::var;

#[throws(Error)]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = OkExRest::with_credential(
        &var("OKEX_KEY")?,
        &var("OKEX_SECRET")?,
        &var("OKEX_PASSPHRASE")?,
    );

    let [resp] = client.request(BalanceRequest::multiple(&["BTC"])).await?;

    println!("{:?}", resp);

    let mut req = OrderRequest::limit("IOTA-USDT", TradeMode::Cross, Side::Buy, 2., 10.);
    req.set_ccy("USDT");

    let [resp] = client.request(req).await?;

    println!("{:?}", resp);

    let req = AmendOrderRequest::new_qty("IOTA-USDT", &resp.ord_id, 20.);
    let resp1 = client.request(req).await?;

    println!("{:?}", resp1);

    let req = CancelOrderRequest::with_ord_id("IOTA-USDT", &resp.ord_id);
    let resp = client.request(req).await?;

    println!("{:?}", resp);
}
