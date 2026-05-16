use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceRawResponse {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "p")]
    pub last_price: String,

    #[serde(rename = "q")]
    pub last_quantity: String,

    #[serde(rename = "T")]
    pub timestamp: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(rename = "M")]
    pub ignore: bool,
}


#[derive(Serialize)]
pub struct SubMessageBinance {
    pub method: String,
    pub params: Vec<String>,
    pub id: u32,
}

