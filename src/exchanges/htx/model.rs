use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SubMessageHtx {
    pub sub: String,
    pub id: String,
}

#[derive(Serialize)]
pub struct UnSubMessageHtx {
    pub unsub: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtxRawResponse {
    pub ch: String,
    pub ts: u64,
    pub tick: HtxTick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtxTick {
    pub id: u64,
    pub ts: u64,
    pub data: Vec<HtxTradeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtxTradeData {
    pub id: serde_json::Value,
    pub ts: u64,
    #[serde(rename = "tradeId")]
    pub trade_id: u64,
    pub amount: f64,
    pub price: f64,
    pub direction: String,
}
