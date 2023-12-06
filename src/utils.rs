use anyhow::{anyhow, Result, Context};
use std::time::{Duration, Instant};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;



pub fn format_pair(exchange: &str, pair: &str) -> Result<String> {
    let mut split = pair.split('-');
    let first = split.next().ok_or_else(|| anyhow!("Invalid pair"))?;
    let second = split.next().ok_or_else(|| anyhow!("Invalid pair"))?;

    match exchange.to_lowercase().as_str() {
        "kucoin" | "okex" | "kraken" => Ok(format!("{}-{}", first.to_uppercase(), second.to_uppercase())),
        "bitfinex" => Ok(format!("{}{}", first.to_lowercase(), second.to_lowercase().replace("usdt", "ust"))),
        "binance" | "huobi" | "hitbtc" | "gemini" => Ok(format!("{}{}", first.to_uppercase(), second.to_uppercase())),
        "cex" => Ok(format!("{}/{}", first.to_uppercase(), second.to_uppercase())),
        "coinbase" => Ok(format!("{}-{}", first.to_lowercase(), second.to_lowercase())),
        "gate" => Ok(format!("{}_{}", first.to_lowercase(), second.to_lowercase())),
        "cex" => Ok(format!("{}/{}", first.to_uppercase(), second.to_uppercase())),
        // Ajoutez des cas supplémentaires pour les autres échanges
        _ => Err(anyhow!("Unsupported exchange : {}", exchange)),
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

#[derive(Deserialize, Serialize, Debug)]
pub struct HuobiResponse {
    ch: String,
    status: String,
    ts: u64,
    tick: HuobiTick,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HuobiTick {
    open: f64,
    close: f64,
    low: f64,
    high: f64,
    amount: f64,
    vol: f64,
    count: i64,
    bid: Vec<f64>,
    ask: Vec<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CexResponse {
    timestamp: String,
    low: String,
    high: String,
    last: String,
    volume: String,
    volume30d: String,
    bid: f64,
    ask: f64,
    price_change: String,
    price_change_percentage: String,
    pair: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CoinbaseResponse {
    ask: String,
    bid: String,
    volume: String,
    trade_id: i64,
    price: String,
    size: String,
    time: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KrakenResponse {
    error: Vec<String>,
    result: KrakenResult,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KrakenResult {
    #[serde(flatten)]
    pub pairs: std::collections::HashMap<String, KrakenPairData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KrakenPairData {
    a: Vec<String>,
    b: Vec<String>,
    c: Vec<String>,
    v: Vec<String>,
    p: Vec<String>,
    t: Vec<i64>,
    l: Vec<String>,
    h: Vec<String>,
    o: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GateApiResponse {
    quote_volume: String,
    base_volume: String,
    highest_bid: String,
    high24hr: String,
    last: String,
    lowest_ask: String,
    elapsed: String,
    result: String,
    low24hr: String,
    percent_change: String,
}
