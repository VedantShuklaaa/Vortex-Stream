use crate::{exchanges::okx::model::OkxRawResponse, models::normalized::NormalizedResponse};

pub fn normalize_okx_response(raw: OkxRawResponse) -> NormalizedResponse {
    let trade = raw.data.first().unwrap();

    NormalizedResponse {
        exchange: "okx".to_string(),
        symbol: trade.inst_id.replace("-", "/"),
        event_type: "trade".to_string(),
        event_time: trade.timestamp.clone(),
        trade_id: trade.trade_id.clone(),
        last_price: trade.price.clone(),
        quantity: trade.size.clone(),
        is_buyer_maker: Some(trade.side == "sell"),
        timestamp: trade.timestamp.parse().unwrap(),
    }
}
