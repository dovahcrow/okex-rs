use anyhow::Error;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use okex::websocket::{Channel, Command, Message, OkExWebsocket};
use std::env::var;

#[throws(Error)]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut client = OkExWebsocket::with_credential(
        &var("OKEX_KEY")?,
        &var("OKEX_SECRET")?,
        &var("OKEX_PASSPHRASE")?,
    )
    .await?;

    client
        .send(Command::subscribe(vec![Channel::Tickers {
            inst_id: "BTC-USDT".to_string(),
        }]))
        .await?;

    while let Some(x) = client.next().await {
        println!("{:?}", x)
    }
}
