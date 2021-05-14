use anyhow::Error;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use okex::websocket::{models::PriceLimit, Channel, Command, Message, OkExWebsocket};
use serde_json::from_value;

#[throws(Error)]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut client = OkExWebsocket::new().await?;

    client
        .send(Command::subscribe(vec![Channel::price_limit(
            "XCH-USDT-SWAP",
        )]))
        .await?;

    while let Some(x) = client.next().await {
        match x.unwrap() {
            Message::Data { arg, mut data, .. } => {
                assert!(matches!(arg, Channel::PriceLimit { .. }));
                let data = data.pop().unwrap();
                let x: PriceLimit = from_value(data).unwrap();
                println!("{:?}", x)
            }
            Message::Error { code, msg, .. } => {
                println!("Error {}: {}", code, msg)
            }
            Message::Event { .. } => {}
            _ => unreachable!(),
        }
    }
}
