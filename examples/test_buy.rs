use anyhow::Error;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use okex::websocket::{Channel, Command, OkExWebsocket};
use okex::{
    enums::{InstType, Side, TdMode},
    rest::{AmendOrderRequest, BalanceRequest, CancelOrderRequest, OkExRest, OrderRequest},
};
use std::env::var;
use std::time::Duration;
use tokio::time::sleep;

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

    let jh = tokio::spawn(async {
        let mut client = OkExWebsocket::with_credential(
            &var("OKEX_KEY")?,
            &var("OKEX_SECRET")?,
            &var("OKEX_PASSPHRASE")?,
        )
        .await?;
        client.send(Command::login(&client)?).await?;
        client.next().await;

        client
            .send(Command::subscribe(vec![Channel::Orders {
                inst_type: InstType::Any,
                uly: None,
                inst_id: Some("IOTA-USDT".to_string()),
            }]))
            .await?;

        while let Some(m) = client.next().await {
            println!("Subscription: {:?}", m);
        }

        Result::<(), Error>::Ok(())
    });
    sleep(Duration::from_secs(3)).await;

    let [resp] = client.request(BalanceRequest::multiple(&["BTC"])).await?;

    println!("{:?}", resp);

    let mut req = OrderRequest::limit("IOTA-USDT", TdMode::Cross, Side::Buy, 1.8, 10.);
    req.set_ccy("USDT");

    let [resp] = client.request(req).await?;

    println!("{:?}", resp);

    let req = AmendOrderRequest::new_qty("IOTA-USDT", &resp.ord_id, 20.);
    let resp1 = client.request(req).await?;

    println!("{:?}", resp1);

    let req = CancelOrderRequest::with_ord_id("IOTA-USDT", &resp.ord_id);
    let resp = client.request(req).await?;

    println!("{:?}", resp);

    jh.await?
}
