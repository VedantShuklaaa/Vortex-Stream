use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkxRawResponse {
    pub arg: Option<OkxTradeArg>,
    #[serde(default)]
    pub data: Vec<OkxResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkxTradeArg {
    pub channel: String,

    #[serde(rename = "instId")]
    pub inst_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkxResponse {
    #[serde(rename = "instId")]
    pub inst_id: String,

    #[serde(rename = "tradeId")]
    pub trade_id: String,

    #[serde(rename = "px")]
    pub price: String,

    #[serde(rename = "sz")]
    pub size: String,

    pub side: String,

    #[serde(rename = "ts")]
    pub timestamp: String,

    pub count: String,

    pub source: String,

    #[serde(rename = "seqId")]
    pub seq_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubMessageOkx {
	pub op: String,
    pub args: Vec<OkxSubscriptionArg>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OkxSubscriptionArg {
    pub channel: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
}