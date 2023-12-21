use anyhow::{anyhow, Result};
use std::time::{Duration, Instant};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use chrono::{DateTime, NaiveDateTime, Utc, Time}; // Import the Utc trait


pub fn format_pair(exchange: &str, pair: &str) -> Result<String> {
    let mut split = pair.split('-');
    let first = split.next().ok_or_else(|| anyhow!("Invalid pair"))?;
    let second = split.next().ok_or_else(|| anyhow!("Invalid pair"))?;

    let formatted_pair = match exchange.to_lowercase().as_str() {
        "kucoin" | "okex" => format!("{}-{}", first.to_uppercase(), second.to_uppercase()),
        "bitfinex" => format!("{}{}", first.to_lowercase(), second.to_lowercase().replace("usdt", "ust")),
        "binance" | "hitbtc" | "gemini" | "kraken" => format!("{}{}", first.to_uppercase(), second.to_uppercase()),
        "cex" => format!("{}/{}", first.to_uppercase(), second.to_uppercase()),
        "coinbase" => format!("{}-{}", first.to_lowercase(), second.to_lowercase()),
        "gate" => format!("{}_{}", first.to_lowercase(), second.to_lowercase()),
        "huobi" => format!("{}{}", first.to_lowercase(), second.to_lowercase()),
        _ => return Err(anyhow!("Unsupported exchange : {}", exchange)),
    };

    // println!("Formatted pair for {} : {}", exchange, formatted_pair);

    Ok(formatted_pair)
}

pub async fn fetch_data(url: &str) -> Result<((String, String), Duration)> {
    let client = reqwest::Client::new();
    let start_time = Instant::now();

    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .text()
        .await?;

    // if url.contains("gate") {
    //     println!("gate Response: {}", response);
    // }
    let parsed_data = if url.contains("kucoin.com") {
        let data: KucoinData = serde_json::from_str(&response)?;
        //println!("Kucoin Data: {:?}", data);
        (data.data.best_ask, data.data.best_bid)
    } else if url.contains("bitfinex.com") {
        let data: BitfinexResponse = serde_json::from_str(&response)?;
        //println!("Bitfinex Data: {:?}", data);
        (data.ask, data.bid)
    } else if url.contains("binance.com") {
        let data: BinanceResponse = serde_json::from_str(&response)?;
        //println!("Binance Data: {:?}", data);
        (data.bids[0][0].clone(), data.asks[0][0].clone())
    } else if url.contains("huobi.pro") {
        let data: HuobiResponse = serde_json::from_str(&response)?;
        (data.tick.ask[0].to_string(), data.tick.bid[0].to_string())
    } else if url.contains("cex.io") {
        let data: CexResponse = serde_json::from_str(&response)?;
        //println!("Cex Data: {:?}", data);
        (data.bid.to_string(), data.ask.to_string())
    } else if url.contains("coinbase.com") {
        let data: CoinbaseResponse = serde_json::from_str(&response)?;
        //println!("Coinbase Data: {:?}", data);
        (data.bid, data.ask)
    } else if url.contains("kraken.com") {
        let data: KrakenResponse = serde_json::from_str(&response)?;
        // println!("Kraken Data: {:?}", data);
        // Exemple pour Kraken, ajustez selon la paire de devises spécifique
        let pair_data = data.result.pairs.get("ETHUSDT").unwrap();
        (pair_data.b[0].clone(), pair_data.a[0].clone())
    } else if url.contains("gateapi.io") {
        let data: GateApiResponse = serde_json::from_str(&response)?;
        //println!("Gate Data: {:?}", data);
        (data.highest_bid, data.lowest_ask)
    } else {
        return Err(anyhow!("Unsupported URL"));
    };

    let duration = start_time.elapsed();
    Ok((parsed_data, duration))
}

// fn convert_unix_timestamp(timestamp: u64) -> String {
//     let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).expect("Invalid timestamp");
//     let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);
//     datetime.format("%Y-%m-%d %H:%M:%S").to_string()
// }


    
// Structures de données pour les réponses des différentes API
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
    #[serde(rename = "lastUpdateId")]
    last_update_id: u64,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
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
    id: u64,
    version: u64,
    open: f64,
    close: f64,
    low: f64,
    high: f64,
    amount: f64,
    vol: f64,
    count: u64,
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
    #[serde(rename = "priceChange")]
    price_change: String,
    #[serde(rename = "priceChangePercentage")]
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
    #[serde(rename = "quoteVolume")]
    quote_volume: String,
    #[serde(rename = "baseVolume")]
    base_volume: String,
    #[serde(rename = "highestBid")]
    highest_bid: String,
    high24hr: String,
    last: String,
    #[serde(rename = "lowestAsk")]
    lowest_ask: String,
    elapsed: String,
    result: String,
    low24hr: String,
    #[serde(rename = "percentChange")]
    percent_change: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketData {
    pub timestamp: String,
    pub name: String,
    pub duration: Duration,
    pub highest_bid: String,
    pub lowest_ask: String,
}