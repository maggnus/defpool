use async_trait::async_trait;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::{debug, warn};

/// Abstract interface for fetching coin prices
#[async_trait]
pub trait PriceProvider: Send + Sync {
    /// Get the price of a coin in BTC
    async fn get_price_btc(&self, coin: &str) -> Result<f64>;
}

/// CoinGecko API price provider
pub struct CoinGeckoProvider {
    client: reqwest::Client,
    coin_id_map: HashMap<String, String>,
}

impl CoinGeckoProvider {
    pub fn new() -> Self {
        let mut coin_id_map = HashMap::new();
        // Map coin symbols to CoinGecko IDs
        coin_id_map.insert("XMR".to_string(), "monero".to_string());
        coin_id_map.insert("BTC".to_string(), "bitcoin".to_string());
        coin_id_map.insert("ETH".to_string(), "ethereum".to_string());
        coin_id_map.insert("LTC".to_string(), "litecoin".to_string());

        Self {
            client: reqwest::Client::new(),
            coin_id_map,
        }
    }

    fn get_coin_id(&self, symbol: &str) -> Option<&str> {
        self.coin_id_map.get(symbol).map(|s| s.as_str())
    }
}

#[derive(Deserialize)]
struct CoinGeckoResponse {
    #[serde(flatten)]
    prices: HashMap<String, CoinPrice>,
}

#[derive(Deserialize)]
struct CoinPrice {
    btc: f64,
}

#[async_trait]
impl PriceProvider for CoinGeckoProvider {
    async fn get_price_btc(&self, coin: &str) -> Result<f64> {
        let coin_id = self.get_coin_id(coin)
            .ok_or_else(|| anyhow::anyhow!("Unknown coin: {}", coin))?;

        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=btc",
            coin_id
        );

        debug!("Fetching price for {} from CoinGecko", coin);

        let response = self.client
            .get(&url)
            .header("User-Agent", "DefPool/1.0")
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("CoinGecko API returned status: {}", response.status());
            anyhow::bail!("CoinGecko API error: {}", response.status());
        }

        let data: CoinGeckoResponse = response.json().await?;
        
        let price = data.prices
            .get(coin_id)
            .ok_or_else(|| anyhow::anyhow!("Price not found for {}", coin))?
            .btc;

        debug!("Price for {}: {} BTC", coin, price);
        Ok(price)
    }
}

impl Default for CoinGeckoProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock price provider for testing
#[allow(dead_code)]
pub struct MockPriceProvider {
    default_price: f64,
}

#[allow(dead_code)]
impl MockPriceProvider {
    pub fn new(default_price: f64) -> Self {
        Self { default_price }
    }
}

#[async_trait]
impl PriceProvider for MockPriceProvider {
    async fn get_price_btc(&self, _coin: &str) -> Result<f64> {
        Ok(self.default_price)
    }
}

