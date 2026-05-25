use crate::{
    exchanges::coinbase::model::CoinbaseRawResponse,
    models::normalized::NormalizedResponse,
};

pub fn normalize_coinbase_response(
    raw: CoinbaseRawResponse,
) -> NormalizedResponse {
    let timestamp =
        raw.timestamp.timestamp_millis() as u64;

    NormalizedResponse {
        exchange: "coinbase".to_string(),
        symbol: raw.symbol.replace("-", "/"),
        event_type: "trade".to_string(),
        event_time: timestamp.to_string(),
        trade_id: raw.trade_id.to_string(),
        last_price: raw.last_price,
        quantity: raw.quantity,
        is_buyer_maker: Some(raw.side == "sell"),
        timestamp,
    }
}