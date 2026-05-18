use crate::{exchanges::bybit::model::BybitRawResponse, models::normalized::NormalizedResponse};

pub fn normalize_bybit_response(raw: BybitRawResponse) -> Vec<NormalizedResponse> {
    raw.data
        .into_iter()
        .map(|trade| NormalizedResponse {
            exchange: "bybit".to_string(),
            symbol: trade.symbol,
            event_type: "trade".to_string(),
            event_time: trade.timestamp.to_string(),
            trade_id: trade.trade_id,
            last_price: trade.price,
            quantity: trade.volume,
            is_buyer_maker: Some(trade.side == "Sell"),
            timestamp: trade.timestamp,
        })
        .collect()
}
