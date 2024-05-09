use std::sync::Arc;

use my_web_socket_client::WsConnection;

use super::models::*;

#[async_trait::async_trait]
pub trait BinanceEventHandler {
    async fn on_data(&self, event: BinanceDataEvent);
    async fn on_connected(&self, connection: Arc<WsConnection>);
    async fn on_disconnected(&self, connection: Arc<WsConnection>);
}
