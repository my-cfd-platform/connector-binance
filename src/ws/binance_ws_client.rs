use my_web_socket_client::WebSocketClient;
use rust_extensions::Logger;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::binance_client_callback::BinanceClientCallback;
use super::binance_ws_settings::BinanceWsSetting;
use super::event_handler::*;
use tokio_tungstenite::tungstenite::Message;

pub struct BinanceWsClient {
    ws_client: WebSocketClient,
    is_started: AtomicBool,
    binance_client_callback: Arc<BinanceClientCallback>,
}

impl BinanceWsClient {
    pub fn new(
        event_handler: Arc<dyn EventHandler + Send + Sync + 'static>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
        instruments: Vec<String>,
    ) -> Self {
        let settings = Arc::new(BinanceWsSetting {});
        Self {
            ws_client: WebSocketClient::new("Binance".to_string(), settings, logger.clone()),

            is_started: AtomicBool::new(false),
            binance_client_callback: Arc::new(BinanceClientCallback::new(
                instruments,
                logger,
                event_handler.clone(),
            )),
        }
    }

    pub fn start(&self) {
        if !self.is_started.load(std::sync::atomic::Ordering::Relaxed) {
            let ping_message = Message::Ping(vec![]);
            self.ws_client
                .start(ping_message, self.binance_client_callback.clone());
            self.is_started
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
