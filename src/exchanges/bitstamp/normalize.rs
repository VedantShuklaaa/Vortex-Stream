use crate::{
    exchanges::bitstamp::model::BitstampRawResponse, models::normalized::NormalizedResponse,
};

pub fn normalize_bitstamp_response(raw: BitstampRawResponse) -> Vec<NormalizedResponse> {
    let data = match raw.data {
        Some(data) => data,
        None => return vec![],
    };

    let symbol = raw
        .channel
        .replace("live_trades_", "")
        .replace("usd", "usdt")
        .to_uppercase();

    vec![NormalizedResponse {
        exchange: "bitstamp".to_string(),
        symbol,
        event_type: "trade".to_string(),
        event_time: data.timestamp.clone(),
        trade_id: data.id.to_string(),
        last_price: data.price_str,
        quantity: data.amount_str,
        is_buyer_maker: Some(data.trade_type == 1),
        timestamp: data.microtimestamp.parse::<u64>().unwrap_or(0) / 1000,
    }]
}
