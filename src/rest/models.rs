use std::fmt;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookSnapshot {
    pub last_update_id: u64,
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kline {
    pub open_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: i64,
    pub quote_asset_volume: String,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
}


#[derive(Debug, Deserialize)]
pub struct KlineData(
    i64,    // 0 open_time
    String, // 1 open
    String, // 2 high
    String, // 3 low
    String, // 4 close
    String, // 5 volume
    i64,    // 6 close_time
    String, // 7 quote_asset_volume
    i64,    // 8 number_of_trades
    String, // 9 taker_buy_base_asset_volume
    String, // 10 taker_buy_quote_asset_volume
    String, // 11 not used
);

impl From<KlineData> for Kline {
    fn from(kline_data: KlineData) -> Self {
        Self {
            open_time: kline_data.0,
            open: kline_data.1,
            high: kline_data.2,
            low: kline_data.3,
            close: kline_data.4,
            volume: kline_data.5,
            close_time: kline_data.6,
            quote_asset_volume: kline_data.7,
            number_of_trades: kline_data.8,
            taker_buy_base_asset_volume: kline_data.9,
            taker_buy_quote_asset_volume: kline_data.10,
        }
    }
}


#[derive(Debug)]
pub enum KlineInterval {
    I1m,
    I3m,
    I5m,
    I15m,
    I30m,
    I1h,
    I2h,
    I4h,
    I6h,
    I8h,
    I12h,
    I1d,
    I3d,
    I1w,
    I1M,
}

impl fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self).replace("I", "");
        write!(f, "{}", name)
    }
}