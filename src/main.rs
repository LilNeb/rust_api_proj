use reqwest;
use std::error::Error;
use std::time::Duration;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let doge = fetch_data().await?;
    println!("{:}", doge);
    
    let urls = vec![
        "https://api.kucoin.com/api/v1/market/orderbook/level1?symbol=ETH-USDT".to_string(),
        "https://api.bitfinex.com/v1/pubticker/ethusd".to_string(),
        "https://api1.binance.com/api/v3/ticker/price?symbol=ETHUSDT".to_string(),
    ];
    
    spawn_threads(urls);
    
    Ok(())
}

async fn fetch_data() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let doge = client
        .get("https://api.kucoin.com/api/v1/market/orderbook/level1?symbol=ETH-USDT")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    Ok(doge)
}


fn spawn_threads(urls: Vec<String>) {
    let mut handles = vec![];

    for (i, url) in urls.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            println!("Thread {} started for URL: {}", i, url);
            // Simuler une opération longue
            thread::sleep(Duration::from_secs(2));
            println!("Thread {} finished for URL: {}", i, url);
        });
        handles.push(handle);
    }

    // Attendre que tous les threads aient terminé
    for handle in handles {
        handle.join().unwrap();
    }
}
