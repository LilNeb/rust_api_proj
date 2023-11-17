use anyhow::{anyhow, Result};

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
