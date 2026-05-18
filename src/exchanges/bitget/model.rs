use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetRawResponse {
    pub action: String,
    pub arg: Args,
    pub data: Vec<BitgetTradeData>,
    pub ts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetTradeData {
    pub ts: String,
    pub price: String,
    pub size: String,
    pub side: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMessageBitget {
    pub op: String,
    pub args: Vec<Args>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Args {
    #[serde(rename = "instType")]
    pub inst_type: String,
    pub channel: String,
    #[serde(rename = "instId")]
    pub inst_id: String
}
