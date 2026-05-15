use crate::{
    exchanges::coinbase::model::CoinbaseRawResponse, models::normalized::NormalizedResponse,
};

pub fn normalize_coinbase_response(raw: CoinbaseRawResponse) -> NormalizedResponse {
	let dt = raw.timestamp;
	let timestamp = dt.timestamp_millis() as u64;
	
    NormalizedResponse {
        exchange: "coinbase".to_string(),
        symbol: raw.symbol.replace("-USD", "USD"),
        event_type: raw.event_type,
        event_time: timestamp.to_string(),
        trade_id: raw.trade_id.to_string(),
        last_price: raw.last_price,
        quantity: raw.quantity,
		is_buyer_maker: None,
        timestamp: timestamp,
    }
}
