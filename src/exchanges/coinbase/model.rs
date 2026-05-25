use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinbaseRawResponse {
    #[serde(rename = "type")]
    pub event_type: String,

    #[serde(rename = "sequence")]
    pub sequence: u64,

    #[serde(rename = "product_id")]
    pub symbol: String,

    #[serde(rename = "price")]
    pub last_price: String,

    #[serde(rename = "open_24h")]
    pub open_24h: String,

    #[serde(rename = "volume_24h")]
    pub volume_24h: String,

    #[serde(rename = "low_24h")]
    pub low_24h: String,

    #[serde(rename = "high_24h")]
    pub high_24h: String,

    #[serde(rename = "volume_30d")]
    pub volume_30d: String,

    #[serde(rename = "best_bid")]
    pub best_bid: String,

    #[serde(rename = "best_bid_size")]
    pub best_bid_size: String,

    #[serde(rename = "best_ask")]
    pub best_ask: String,

    #[serde(rename = "best_ask_size")]
    pub best_ask_size: String,

    #[serde(rename = "side")]
    pub side: String,

    #[serde(rename = "time")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "trade_id")]
    pub trade_id: u64,

    #[serde(rename = "last_size")]
    pub quantity: String,
}

#[derive(Serialize)]
pub struct SubMessageCoinbase {
    #[serde(rename = "type")]
    pub r#type: String,

    pub product_ids: Vec<String>,
    pub channels: Vec<String>,
}
