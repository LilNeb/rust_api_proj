use reqwest;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let doge = client
        .get("https://api.kucoin.com/api/v1/market/orderbook/level1?symbol=ETH-USDT")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", doge);
    Ok(())
}