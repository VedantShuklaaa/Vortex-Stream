use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BybitRawResponse {
    pub topic: String,
    pub ts: u64,

    #[serde(rename = "type")]
    pub event_type: String,
    pub data: Vec<BybitTradeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BybitTradeData {
    #[serde(rename = "i")]
    pub trade_id: String,

    #[serde(rename = "T")]
    pub timestamp: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "s")]
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMessageBybit {
	pub op: String,
	pub args: Vec<String>,
}