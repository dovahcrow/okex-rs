use anyhow::Error;
use fehler::throws;
use okex::{
    enums::InstType,
    rest::{OkExRest, PositionsRequest},
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

    let resp = client
        .request(PositionsRequest::inst_type(InstType::Any))
        .await?;

    println!("{:?}", resp);
}
