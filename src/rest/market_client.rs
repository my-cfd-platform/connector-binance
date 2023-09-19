use chrono::{DateTime, Utc};

use super::{
    enpoints::{BinanceApiEndpoint, BinanceSpotEnpoint},
    models::OrderbookSnapshot,
    rest_client::RestClient,
    util::build_request,
    Kline, KlineInterval, KlineData,
};
use crate::rest::errors::Result;
use std::{collections::BTreeMap};

#[derive(Clone)]
pub struct BinanceMarketClient {
    pub client: RestClient,
    pub recv_window: u64,
}

// Market Data endpoints
impl BinanceMarketClient {
    // Order book at the default depth of 100
    pub async fn get_depth<S>(&self, symbol: S) -> Result<OrderbookSnapshot>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(
                BinanceApiEndpoint::Spot(BinanceSpotEnpoint::Depth),
                Some(request),
            )
            .await
    }

    // Order book at a custom depth. Currently supported values
    // are 5, 10, 20, 50, 100, 500, 1000 and 5000
    pub async fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderbookSnapshot>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("limit".into(), depth.to_string());
        let request = build_request(parameters);
        self.client
            .get(
                BinanceApiEndpoint::Spot(BinanceSpotEnpoint::Depth),
                Some(request),
            )
            .await
    }

    // Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    // https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub async fn get_klines(
        &self,
        symbol: impl Into<String>,
        interval: KlineInterval,
        limit: u16,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<Kline>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.to_string());
        parameters.insert("limit".into(), limit.to_string());
        if let Some(start_time) = start_time {
            parameters.insert("startTime".into(), start_time.timestamp_millis().to_string());
        }
        if let Some(end_time) = end_time {
            parameters.insert("endTime".into(), end_time.timestamp_millis().to_string());
        }

        let request = build_request(parameters);
        let data: Vec<KlineData> = self
            .client
            .get(
                BinanceApiEndpoint::Spot(BinanceSpotEnpoint::Klines),
                Some(request),
            )
            .await
            .expect("Failed to get_klines");
        let klines: Vec<Kline> = data.into_iter().map(|data| data.into()).collect();

        Ok(klines)
    }
}