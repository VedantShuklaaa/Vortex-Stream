use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NormalizedResponse {
	pub exchange: String,
	pub symbol: String,
	pub last_price: String,
	pub quantity: String,
	pub timestamp: u64,
}

#[derive(Serialize)]
pub struct SubscribeMessage {
    pub method: String,
    pub params: Vec<String>,
    pub id: u32,
}