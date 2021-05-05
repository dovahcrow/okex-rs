use anyhow::Error;
use fehler::throws;
use okex::{
    enums::InstType,
    rest::{InstrumentsRequest, OkExRest},
};

#[throws(Error)]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = OkExRest::new();

    let resp = client
        .request(InstrumentsRequest::inst_id(InstType::Swap, "LPT-USDT-SWAP"))
        .await?;

    println!("{:?}", resp);

    let resp = client
        .request(InstrumentsRequest::inst_id(InstType::Margin, "LPT-USDT"))
        .await?;

    println!("{:?}", resp);

    let resp = client
        .request(InstrumentsRequest::inst_id(
            InstType::Swap,
            "IOTA-USDT-SWAP",
        ))
        .await?;

    println!("{:?}", resp);

    let resp = client
        .request(InstrumentsRequest::inst_id(InstType::Spot, "IOTA-USDT"))
        .await?;

    println!("{:?}", resp);
}
