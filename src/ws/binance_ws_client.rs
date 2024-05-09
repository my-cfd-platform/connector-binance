use my_web_socket_client::hyper_tungstenite::tungstenite::Message;
use my_web_socket_client::WebSocketClient;
use rust_extensions::Logger;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::binance_client_callback::BinanceClientCallback;
use super::binance_ws_settings::BinanceWsSetting;
use super::event_handler::*;

pub struct BinanceWsClient {
    ws_client: WebSocketClient,
    is_started: AtomicBool,
    binance_client_callback: Arc<BinanceClientCallback>,
}

impl BinanceWsClient {
    pub fn new(
        event_handler: Arc<dyn BinanceEventHandler + Send + Sync + 'static>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
    ) -> Self {
        let settings = Arc::new(BinanceWsSetting {});
        Self {
            ws_client: WebSocketClient::new("Binance".to_string(), settings, logger.clone()),

            is_started: AtomicBool::new(false),
            binance_client_callback: Arc::new(BinanceClientCallback::new(
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

    pub fn stop(&self) {
        self.ws_client.stop();
    }
}
