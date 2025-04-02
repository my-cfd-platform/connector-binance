use std::sync::Arc;

use my_web_socket_client::{hyper_tungstenite::tungstenite::{Bytes, Message}, WsCallback, WsConnection};
use rust_extensions::Logger;
use serde_json::Error;

use super::{BinanceDataEvent, BinanceEventHandler, BinanceOrderBookTopTickers, BookTickerData};

pub struct BinanceClientCallback {
    event_handler: Arc<dyn BinanceEventHandler + Send + Sync + 'static>,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
}

impl BinanceClientCallback {
    pub fn new(
        logger: Arc<dyn Logger + Send + Sync + 'static>,
        event_handler: Arc<dyn BinanceEventHandler + Send + Sync + 'static>,
    ) -> Self {
        Self {
            logger,
            event_handler,
        }
    }
}

#[async_trait::async_trait]
impl WsCallback for BinanceClientCallback {
    async fn on_connected(&self, connection: Arc<WsConnection>) {
        self.logger.write_info(
            "BinanceWsClient".to_string(),
            "Connected to Binance websocket".to_string(),
            None,
        );

        let event_handler = self.event_handler.clone();

        let result = tokio::spawn(async move {
            event_handler.on_connected(connection).await;
        })
        .await;

        if result.is_err() {
            self.logger.write_error(
                "BinanceWsClient".to_string(),
                "Panic in on_connected event".to_string(),
                None,
            );
        }
    }

    async fn on_disconnected(&self, connection: Arc<WsConnection>) {
        let event_handler = self.event_handler.clone();

        let result = tokio::spawn(async move {
            event_handler.on_connected(connection).await;
        })
        .await;
        if result.is_err() {
            self.logger.write_error(
                "BinanceWsClient".to_string(),
                "Panic in on_disconnected event".to_string(),
                None,
            );
        }
    }

    async fn on_data(&self, connection: Arc<WsConnection>, data: Message) {
        match data {
            Message::Text(msg) => {
                let event = parse_msg(&msg);

                self.event_handler.on_data(event).await;
            }
            Message::Ping(_) => {
                connection.send_message(Message::Ping(Bytes::default())).await;
            }
            Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => (),
            Message::Close(_) => {
                self.logger.write_info(
                    "BinanceWsClient".to_string(),
                    format!("Disconnecting... Received close ws message"),
                    None,
                );
            }
        }
    }
}

fn parse_msg(msg: &str) -> BinanceDataEvent {
    let value: Result<serde_json::Value, Error> = serde_json::from_str(msg);

    let Ok(value) = value else {
        return BinanceDataEvent::Unknown(msg.to_string());
    };

    let Some(stream) = value.get("stream") else {
        return BinanceDataEvent::Unknown(msg.to_string());
    };

    let Some(data) = value.get("data") else {
        return BinanceDataEvent::Unknown(msg.to_string());
    };

    let ticker = stream
        .to_string()
        .split("@")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let data: BinanceOrderBookTopTickers = serde_json::from_str(&data.to_string()).unwrap();

    let ticker = BookTickerData {
        update_id: data.last_update_id as u64,
        symbol: ticker.first().unwrap().to_string().replace("\"", ""),
        best_bid: data.bids.first().unwrap()[0].to_string(),
        best_bid_qty: data.bids.first().unwrap()[1].to_string(),
        best_ask: data.asks.first().unwrap()[0].to_string(),
        best_ask_qty: data.asks.first().unwrap()[1].to_string(),
    };

    return BinanceDataEvent::BookTicker(ticker);
}
