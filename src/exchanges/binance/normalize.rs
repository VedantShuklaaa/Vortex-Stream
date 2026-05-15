use crate::{
	exchanges::binance::model::{BinanceRawResponse},
	models::normalized::{NormalizedResponse},
};


pub fn normalize_binance_response(raw: BinanceRawResponse) -> NormalizedResponse {
    NormalizedResponse {
        exchange: "binance".to_string(),
        symbol: raw.symbol,
		event_type: raw.event_type,
		event_time: raw.event_time.to_string(),
		trade_id: raw.trade_id.to_string(),
        last_price: raw.last_price,
        quantity: raw.last_quantity,
		is_buyer_maker: Some(raw.is_buyer_maker),
        timestamp: raw.timestamp,
    }
}