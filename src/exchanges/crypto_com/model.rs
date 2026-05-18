use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMessageCryptoCom {
    pub id: u64,
    pub method: String,
    pub params: CryptoComParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoComParams {
    pub channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoComRawResponse {
    pub id: i64,
    pub method: String,
    pub code: u64,
    pub result: CryptoComResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoComResult {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    pub subscription: String,
    pub channel: String,
    pub data: Vec<CryptoComTradeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoComTradeData {
    #[serde(rename = "d")]
    pub trade_id: String,

    #[serde(rename = "t")]
    pub timestamp: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub quantity: String,

    #[serde(rename = "s")]
    pub side: String,

    #[serde(rename = "i")]
    pub instrument: String,

    #[serde(rename = "m")]
    pub match_id: String,
}
