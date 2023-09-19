use super::{config::Config, market_client::BinanceMarketClient, rest_client::RestClient};


#[allow(clippy::all)]
pub enum BinanceApiEndpoint {
    Spot(BinanceSpotEnpoint),
}

/// Endpoint for production and test orders.
///
/// Orders issued to test are validated, but not sent into the matching engine.
pub enum BinanceSpotEnpoint {
    Ping,
    Depth,
    Klines,
}

impl From<BinanceApiEndpoint> for String {
    fn from(item: BinanceApiEndpoint) -> Self {
        String::from(match item {
            BinanceApiEndpoint::Spot(route) => match route {
                BinanceSpotEnpoint::Ping => "/api/v3/ping",
                BinanceSpotEnpoint::Depth => "/api/v3/depth",
                BinanceSpotEnpoint::Klines => "/api/v3/klines",
            },           
        })
    }
}
pub trait BinanceRestClient {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;
    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self;
}

impl BinanceRestClient for BinanceMarketClient {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> BinanceMarketClient {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> BinanceMarketClient {
        BinanceMarketClient {
            client: RestClient::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}