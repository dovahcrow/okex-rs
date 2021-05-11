use anyhow::Error;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use okex::websocket::{models::Book, Channel, Command, Message, OkExWebsocket};
use serde_json::from_value;

#[throws(Error)]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut client = OkExWebsocket::new().await?;

    client
        .send(Command::subscribe(vec![Channel::books_l2_tbt("BTC-USDT")]))
        .await?;

    while let Some(x) = client.next().await {
        if let Message::Data { mut data, .. } = x.unwrap() {
            assert!(data.len() == 1);
            let data: Book = from_value(data.pop().unwrap()).unwrap();
            println!("{:?}", data)
        }
    }
}
