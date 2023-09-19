use my_web_socket_client::WsClientSettings;

pub struct BinanceWsSetting;

#[async_trait::async_trait]
impl WsClientSettings for BinanceWsSetting {
    async fn get_url(&self) -> String {
        return BinanceWsUrl::MultiStream.params();
    }
}

#[allow(clippy::all)]
enum BinanceWsUrl {
    MultiStream,
}

impl BinanceWsUrl {
    fn params(self) -> String {
        match self {
            BinanceWsUrl::MultiStream => format!("wss://stream.binance.com:9443/stream",),
        }
    }
}
