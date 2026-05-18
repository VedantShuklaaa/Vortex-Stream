use serde::{Serialize, Deserialize};

pub type KrakenRawResponse = (u64, Vec<KrakenTrade>, String, String);

pub type KrakenTrade = (
    String, // price
    String, // volume
    String, // timestamp
    String, // side
    String, // order type
    String, // misc
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrakenSubscription {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMessageKraken {
    pub event: String,
    pub subscription: KrakenSubscription,
    pub pair: Vec<String>,
}
