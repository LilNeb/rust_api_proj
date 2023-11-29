use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use anyhow::Result;
use rust_api_proj::{format_pair, fetch_data};

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    
    let arg = "eth-usdt";
    let kucoin_arg = format_pair("kucoin", arg);
    let bitfinex_arg = format_pair("bitfinex", arg);
    let binance_arg = format_pair("binance", arg);
    println!("Kucoin: {:?}", kucoin_arg);
    println!("Bitfinex: {:?}", bitfinex_arg);
    println!("Binance: {:?}", binance_arg);

    let urls = vec![
        ("Kucoin", format!("https://api.kucoin.com/api/v1/market/orderbook/level1?symbol={}", kucoin_arg.unwrap())),
        ("Bitfinex", format!("https://api.bitfinex.com/v1/pubticker/{}", bitfinex_arg.unwrap())),
        ("Binance", format!("https://api1.binance.com/api/v3/ticker/price?symbol={}", binance_arg.unwrap()))
    ];
    

    let (tx, mut rx) = mpsc::channel::<(String, Duration, String)>(urls.len());

    for (name, url) in urls {
        let tx_clone = tx.clone();
        task::spawn(async move {
            if let Ok((data, duration)) = fetch_data(&url).await {
                let _ = tx_clone.send((format!("{}: {}", name, data), duration, name.to_string())).await;
            }
        });
    }

    drop(tx); // Fermer le canal

    while let Some((data, duration, name)) = rx.recv().await {
        println!("{} received in {:?}: {}", name, duration, data);
    }

    Ok(())}