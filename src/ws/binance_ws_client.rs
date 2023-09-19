use my_web_socket_client::WebSocketClient;
use my_web_socket_client::WsCallback;
use my_web_socket_client::WsConnection;
use rust_extensions::Logger;
use serde_json::Error;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::binance_ws_settings::BinanceWsSetting;
use super::event_handler::*;
use super::models::*;
use tokio_tungstenite::tungstenite::Message;

pub struct BinanceWsClient {
    event_handler: Arc<dyn EventHandler + Send + Sync + 'static>,
    ws_client: WebSocketClient,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
    is_started: AtomicBool,
    instruments_to_subscribe: Vec<String>,
}

impl BinanceWsClient {
    pub fn new(
        event_handler: Arc<dyn EventHandler + Send + Sync + 'static>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
        instruments: Vec<String>,
    ) -> Self {
        let settings = Arc::new(BinanceWsSetting {});
        Self {
            event_handler,
            ws_client: WebSocketClient::new("Binance".to_string(), settings, logger.clone()),
            logger,
            is_started: AtomicBool::new(false),
            instruments_to_subscribe: instruments,
        }
    }

    pub fn start(ftx_ws_client: Arc<BinanceWsClient>) {
        if !ftx_ws_client
            .is_started
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let ping_message = Message::Ping(vec![]);
            ftx_ws_client
                .ws_client
                .start(ping_message, ftx_ws_client.clone());
            ftx_ws_client
                .is_started
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    fn parse_msg(&self, msg: &str) -> Result<WsDataEvent, String> {
        let value: Result<serde_json::Value, Error> = serde_json::from_str(msg);

        let Ok(value) = value else {
            return Err(format!("Failed to parse message: {}", msg));
        };

        let Some(stream) = value.get("stream") else {
            return Err(format!("Failed to parse message: {}", msg));
        };

        let Some(data) = value.get("data") else {
            return Err(format!("Failed to parse message: {}", msg));
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

        return Ok(WsDataEvent::BookTicker(ticker));
    }
}

#[async_trait::async_trait]
impl WsCallback for BinanceWsClient {
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
                let event = self.parse_msg(&msg);
                match event {
                    Ok(event) => {
                        self.event_handler.on_data(event).await;
                    }
                    Err(err) => {
                        println!("error: {}", err)
                    }
                }
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
