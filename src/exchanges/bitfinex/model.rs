use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMessageBitfinex {
    pub event: String,
    pub channel: String,
    pub symbol: String,
}

pub type BitfinexTrade = (
    u64, // trade id
    u64, // timestamp
    f64, // amount
    f64, // price
);

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BitfinexRawResponse {
    // snapshot
    Snapshot(
        u64, // channel id
        Vec<BitfinexTrade>,
    ),

    // te / tu update
    Update(
        u64,    // channel id
        String, // event type
        BitfinexTrade,
    ),
}
