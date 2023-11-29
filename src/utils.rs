use anyhow::{anyhow, Result, Context};
use std::time::{Duration, Instant};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;


pub fn format_pair(exchange: &str, pair: &str) -> Result<String> {
    let mut split = pair.split('-');
    let first = split.next().ok_or_else(|| anyhow!("Invalid pair"))?;
    let second = split.next().ok_or_else(|| anyhow!("Invalid pair"))?;

    match exchange {
        "kucoin" => Ok(format!("{}-{}", first.to_uppercase(), second.to_uppercase())),
        "bitfinex" => {
            let formatted_pair = pair
                .replace("-", "")
                .replace("usdt", "ust")
                .to_lowercase();
            Ok(formatted_pair)
        },
        "binance" => Ok(format!("{}{}", first.to_uppercase(), second.to_uppercase())),
        _ => Err(anyhow!("Invalid exchange")),
    }
}

pub async fn fetch_data(url: &str) -> Result<(String, Duration)> {
    let client = reqwest::Client::new();
    let start_time = Instant::now();

    let response = client
        .get(url)
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

    let duration = start_time.elapsed();

    let parsed_data = if url.contains("kucoin.com") {
        let data: KucoinData = serde_json::from_str(&response)
            .context("Failed to parse Kucoin response")?;
        data.data.best_ask // Exemple: sélectionner le champ 'price' pour Kucoin
    } else if url.contains("bitfinex.com") {
        let data: BitfinexResponse = serde_json::from_str(&response)
            .context("Failed to parse Bitfinex response")?;
        data.ask // Exemple: sélectionner le champ 'last_price' pour Bitfinex
    } else if url.contains("binance.com") {
        let data: BinanceResponse = serde_json::from_str(&response)
            .context("Failed to parse Binance response")?;
        data.price // Exemple: sélectionner le champ 'price' pour Binance
    } else {
        return Err(anyhow!("Unsupported URL"));
    };

    Ok((parsed_data, duration))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KucoinData {
    code: String,
    data: KucoinDataInner,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KucoinDataInner {
    time: u64,
    sequence: String,
    price: String,
    size: String,
    #[serde(rename = "bestBid")]
    best_bid: String,
    #[serde(rename = "bestBidSize")]
    best_bid_size: String,
    #[serde(rename = "bestAsk")]
    best_ask: String,
    #[serde(rename = "bestAskSize")]
    best_ask_size: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BitfinexResponse {
    mid: String,
    bid: String,
    ask: String,
    last_price: String,
    low: String,
    high: String,
    volume: String,
    timestamp: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BinanceResponse {
    symbol: String,
    price: String,
}