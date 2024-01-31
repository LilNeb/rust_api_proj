use std::f32::MAX_10_EXP;

use plotters::prelude::*;
use rusqlite::{Connection, Result};

struct MarketData {
    timestamp: String,
    highest_bid: String,
    lowest_ask: String,
}

struct ParsedMarketData {
    timestamp: i64,
    highest_bid: f64,
    lowest_ask: f64,
}

pub fn get_market_data(conn: &Connection) -> Result<Vec<ParsedMarketData>, rusqlite::Error> {
    let mut stmt = conn
        .prepare("SELECT timestamp, highest_bid, lowest_ask FROM market_data ORDER BY timestamp DESC LIMIT 100")?;

    let market_data_iter = stmt.query_map([], |row| {
        Ok(MarketData {
            timestamp: row.get(0)?,
            highest_bid: row.get(1)?,
            lowest_ask: row.get(2)?,
        })
    })?;

    let mut market_data_list: Vec<ParsedMarketData> = Vec::new();

    for market_data_result in market_data_iter {
        let market_data = market_data_result?;
        // Handle parsing errors by skipping invalid entries or returning an error
        let parsed_market_data = ParsedMarketData {
            timestamp: market_data.timestamp.parse().unwrap(),
            highest_bid: market_data.highest_bid.parse().unwrap(),
            lowest_ask: market_data.lowest_ask.parse().unwrap(),
        };

        market_data_list.push(parsed_market_data);
    }

    Ok(market_data_list)
}

pub fn get_market_data_by_exchange(
    conn: &Connection,
    exchange: &str,
) -> Result<Vec<ParsedMarketData>, rusqlite::Error> {
    let mut stmt = conn
        .prepare(&format!("SELECT timestamp, highest_bid, lowest_ask FROM market_data WHERE name='{}' ORDER BY timestamp DESC LIMIT 20", exchange))?;

    let market_data_iter = stmt.query_map([], |row| {
        Ok(MarketData {
            timestamp: row.get(0)?,
            highest_bid: row.get(1)?,
            lowest_ask: row.get(2)?,
        })
    })?;

    let mut market_data_list: Vec<ParsedMarketData> = Vec::new();

    for market_data_result in market_data_iter {
        let market_data = market_data_result?;
        // Handle parsing errors by skipping invalid entries or returning an error
        let parsed_market_data = ParsedMarketData {
            timestamp: market_data.timestamp.parse().unwrap(),
            highest_bid: market_data.highest_bid.parse().unwrap(),
            lowest_ask: market_data.lowest_ask.parse().unwrap(),
        };

        market_data_list.push(parsed_market_data);
    }

    Ok(market_data_list)
}

pub fn draw_plot(conn: &Connection, exchange: &str) -> Result<()> {
    let market_data_list;

    if (exchange.eq("All")) {
        market_data_list = get_market_data(&conn)?;
    } else {
        market_data_list = get_market_data_by_exchange(&conn, exchange)?;
    }

    let max_market_price: f64 = market_data_list
        .iter()
        .fold(0.0, |acc, itt| acc.max(itt.highest_bid));
    let min_market_price: f64 = market_data_list
        .iter()
        .fold(f64::MAX, |acc, itt| acc.min(itt.lowest_ask));
    let min_date: i64 = market_data_list
        .iter()
        .fold(i64::MAX, |acc, itt| acc.min(itt.timestamp));
    let max_date: i64 = market_data_list
        .iter()
        .fold(0, |acc, itt| acc.max(itt.timestamp));

    // println!("{max_market_price} {min_market_price} {min_date} {max_date}");

    let plot_name = format!("front_end/img/{}-plot.png", exchange);

    let drawing_area = BitMapBackend::new(&plot_name, (1920, 1080)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 80)
        .build_cartesian_2d(min_date..max_date, min_market_price..max_market_price)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            market_data_list
                .iter()
                .map(|x| Circle::new((x.timestamp, x.highest_bid), 2, &RED)),
        )
        .unwrap();

    chart
        .draw_series(
            market_data_list
                .iter()
                .map(|x| Circle::new((x.timestamp, x.lowest_ask), 2, &GREEN)),
        )
        .unwrap();

    Ok(())
}
