use crate::{
    exchanges::bitfinex::model::BitfinexRawResponse, models::normalized::NormalizedResponse,
};

pub fn normalize_bitfinex_response(raw: BitfinexRawResponse) -> Vec<NormalizedResponse> {
    let (_channel_id, event_type, trade) = raw;

    if event_type != "tu" {
        return vec![];
    }
    let (trade_id, timestamp, amount, price) = trade;

    vec![NormalizedResponse {
        exchange: "bitfinex".to_string(),
        symbol: "BTCUSDT".to_string(),
        event_type: "trade".to_string(),
        event_time: timestamp.to_string(),
        trade_id: trade_id.to_string(),
        last_price: price.to_string(),
        quantity: amount.abs().to_string(),
        is_buyer_maker: Some(amount < 0.0),
        timestamp,
    }]
}
