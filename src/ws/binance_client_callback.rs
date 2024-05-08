use std::sync::Arc;

use my_web_socket_client::{hyper_tungstenite::tungstenite::Message, WsCallback, WsConnection};
use rust_extensions::Logger;
use serde_json::Error;

use super::{
    BinanceDataEvent, BinanceOrderBookTopTickers, BinanceSubscribeMessage, BookTickerData,
    EventHandler,
};

pub struct BinanceClientCallback {
    event_handler: Arc<dyn EventHandler + Send + Sync + 'static>,
    pub instruments_to_subscribe: Vec<String>,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
}

impl BinanceClientCallback {
    pub fn new(
        instruments_to_subscribe: Vec<String>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
        event_handler: Arc<dyn EventHandler + Send + Sync + 'static>,
    ) -> Self {
        Self {
            instruments_to_subscribe,
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

        let subscribe_msg = BinanceSubscribeMessage::new(self.instruments_to_subscribe.clone());
        connection
            .send_message(Message::Text(
                serde_json::to_string(&subscribe_msg).unwrap(),
            ))
            .await;
        self.event_handler.on_connected().await;
    }

    async fn on_disconnected(&self, _: Arc<WsConnection>) {}

    async fn on_data(&self, connection: Arc<WsConnection>, data: Message) {
        match data {
            Message::Text(msg) => {
                let event = parse_msg(&msg);

                self.event_handler.on_data(event).await;
            }
            Message::Ping(_) => {
                connection.send_message(Message::Ping(vec![])).await;
            }
            Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => (),
            Message::Close(_) => {
                self.logger.write_info(
                    "BinanceWsClient".to_string(),
                    format!("Disconnecting... Recieved close ws message"),
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
