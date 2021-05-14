use anyhow::Error;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use okex::enums::InstType;
use okex::websocket::{models::Instrument, Channel, Command, Message, OkExWebsocket};
use serde_json::from_value;

#[throws(Error)]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut client = OkExWebsocket::new().await?;

    client
        .send(Command::subscribe(vec![Channel::instruments(
            InstType::Spot,
        )]))
        .await?;

    while let Some(x) = client.next().await {
        match x.unwrap() {
            Message::Data { arg, mut data, .. } => {
                assert!(matches!(arg, Channel::Instruments { .. }));
                for d in data {
                    let x: Instrument = from_value(d).unwrap();
                    println!("{:?}", x)
                }
            }
            Message::Error { code, msg, .. } => {
                println!("Error {}: {}", code, msg)
            }
            Message::Event { .. } => {}
            _ => unreachable!(),
        }
    }
}
