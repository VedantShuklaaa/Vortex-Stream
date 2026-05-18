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

pub type BitfinexRawResponse = (
    u64, // channel id
    String, // event type ("te" / "tu")
    BitfinexTrade,
);
