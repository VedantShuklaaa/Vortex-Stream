use crate::{
    exchanges::crypto_com::model::CryptoComRawResponse, models::normalized::NormalizedResponse,
};

pub fn normalize_crypto_com_response(raw: CryptoComRawResponse) -> Vec<NormalizedResponse> {
    raw.result
        .data
        .into_iter()
        .map(|trade| NormalizedResponse {
            exchange: "crypto_com".to_string(),
            symbol: trade.instrument.replace("_", ""),
            event_type: "trade".to_string(),
            event_time: trade.timestamp.to_string(),
            trade_id: trade.trade_id,
            last_price: trade.price,
            quantity: trade.quantity,
            is_buyer_maker: Some(trade.side == "SELL"),
            timestamp: trade.timestamp,
        })
        .collect()
}
