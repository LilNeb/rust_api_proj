use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use anyhow::Result;
use crate::utils::{format_pair, fetch_data};
use crate::utils::MarketData;
use chrono::Utc;

mod utils;

//TODO : Gérer nouvelles urls dans fetch_data, et renvoyer ask et bid pour les traiter plus tard

#[tokio::main]
async fn main() -> Result<()> {
    

    let arg = "eth-usdt";
    let kucoin_arg = format_pair("kucoin", arg)?;
    let bitfinex_arg = format_pair("bitfinex", arg)?;
    let binance_arg = format_pair("binance", arg)?;
    let cex_arg = format_pair("cex", arg)?;
    let coinbase_arg = format_pair("coinbase", arg)?;
    let kraken_arg = format_pair("kraken", arg)?;
    let huobi_arg = format_pair("huobi", arg)?;
    let gate_arg = format_pair("gate", arg)?;
    let hitbtc_arg = format_pair("hitbtc", arg)?;
    let okex_arg = format_pair("okex", arg)?;


    let urls = vec![
        ("Kucoin", format!("https://api.kucoin.com/api/v1/market/orderbook/level1?symbol={}", kucoin_arg)),
        ("Bitfinex", format!("https://api.bitfinex.com/v1/pubticker/{}", bitfinex_arg)),
        ("Binance", format!("https://api.binance.com/api/v3/depth?limit=10&symbol={}", binance_arg)),
        ("Cex", format!("https://cex.io/api/ticker/{}", cex_arg)),
        ("Coinbase", format!("https://api.pro.coinbase.com/products/{}/ticker", coinbase_arg)),
        ("Kraken", format!("https://api.kraken.com/0/public/Ticker?pair={}", kraken_arg)),
        ("Huobi", format!("https://api.huobi.pro/market/detail/merged?symbol={}", huobi_arg)),
        ("Gate", format!("https://data.gateapi.io/api2/1/ticker/{}", gate_arg)),
        ("Hitbtc", format!("https://api.hitbtc.com/api/2/public/ticker/{}", hitbtc_arg)),
        ("Okex", format!("https://www.okex.com/api/spot/v3/instruments/{}/ticker", okex_arg)),
    ];

    let (tx, mut rx) = mpsc::channel::<(((String, String), Duration), String)>(urls.len());

    for (name, url) in urls {
        let tx_clone = tx.clone();
        task::spawn(async move {
            let thread_number = std::thread::current().id(); // Get the thread number
            if let Ok(((value1, value2), duration)) = fetch_data(&url).await {
                let _ = tx_clone.send((((value1, value2), duration), format!("{} - Thread {:?}", name, thread_number))).await; // Include the thread number in the name
            }
        });
    }
    
    drop(tx); // Close the channel
    
    let mut market_data_list: Vec<MarketData> = Vec::new();

    while let Some((((data1, data2), duration), name)) = rx.recv().await {
        let market_data = MarketData {
            timestamp: Utc::now().timestamp().to_string(),
            name,
            duration,
            highest_bid: data1,
            lowest_ask: data2,
        };
        market_data_list.push(market_data);
    }

    println!("Market Data List: {:?}", market_data_list);
    
    
    Ok(())
}
