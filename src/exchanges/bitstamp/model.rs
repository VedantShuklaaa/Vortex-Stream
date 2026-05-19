use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitstampRawResponse {
    pub data: Option<BitstampTradeData>,
    pub channel: String,
    pub event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitstampTradeData {
    pub id: u64,
    pub timestamp: String,
    pub amount: f64,

    #[serde(rename = "amount_str")]
    pub amount_str: String,

    pub price: f64,

    #[serde(rename = "price_str")]
    pub price_str: String,

    #[serde(rename = "type")]
    pub trade_type: u8,

    pub microtimestamp: String,
    pub buy_order_id: u64,
    pub sell_order_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMessageBitstamp {
    pub event: String,
    pub data: BitstampChannelData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitstampChannelData {
    pub channel: String,
}
