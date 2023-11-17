use reqwest;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();

    let urls = vec![
        ("Kucoin", "https://api.kucoin.com/api/v1/market/orderbook/level1?symbol=ETH-USDT"),
        ("Bitfinex", "https://api.bitfinex.com/v1/pubticker/ethusd"),
        ("Binance", "https://api1.binance.com/api/v3/ticker/price?symbol=ETHUSDT"),
    ];

    let (tx, mut rx) = mpsc::channel::<(String, Duration, String)>(urls.len());

    for (name, url) in urls {
        let tx_clone = tx.clone();
        let url = url.to_string();
        let name = name.to_string();
        task::spawn(async move {
            if let Ok(data) = fetch_data(&url).await {
                let duration = start_time.elapsed();
                tx_clone.send((data, duration, name)).await.unwrap();
            }
        });
    }

    drop(tx); // Fermer le canal

    while let Some((data, duration, name)) = rx.recv().await {
        println!("{} received in {:?}: {}", name, duration, data);
    }

    Ok(())
}

async fn fetch_data(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}
