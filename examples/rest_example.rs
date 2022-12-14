use std::collections::BTreeSet;

use chrono::{Utc, Duration};
use connector_binance::rest::{BinanceMarketClient, BinanceRestClient, KlineInterval};

#[tokio::main]
async fn main() {
    let market: BinanceMarketClient = BinanceRestClient::new(None, None);

    loop {
        //get_orderbook(&market).await;
        get_klines(&market).await;

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}

#[allow(dead_code)]
async fn get_orderbook(market: &BinanceMarketClient) {
    match market.get_custom_depth("BNBETH", 5).await {
        Ok(answer) => println!("Custom depth: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

async fn get_klines(market: &BinanceMarketClient) {
    let end = Utc::now();
    let start = end - Duration::minutes(60);
    println!("start: {}; end: {}", start.timestamp_millis(), end.timestamp_millis());

    match market.get_klines("BNBBUSD", KlineInterval::I1m, 10, Some(start), Some(end)).await {
        Ok(klines) => {
            let dates: BTreeSet<i64> = klines.iter().map(|l| l.open_time).collect();
            println!("dates: {:?}", dates);
        },
        Err(e) => println!("Error: {}", e),
    }
}