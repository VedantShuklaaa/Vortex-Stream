use serde::{Serialize, Deserialize};

/// Unified normalized trade event
/// used internally across exchanges.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NormalizedResponse {
	pub exchange: String,
	pub symbol: String,
	pub event_type: String,
	pub event_time: String,
	pub trade_id: String,
	pub last_price: String,
	pub quantity: String,
	pub is_buyer_maker: bool,
	pub timestamp: u64,
}

#[derive(Serialize)]
pub struct SubscribeMessage {
    pub method: String,
    pub params: Vec<String>,
    pub id: u32,
}