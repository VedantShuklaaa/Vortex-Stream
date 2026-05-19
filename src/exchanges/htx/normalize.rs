use crate::{exchanges::htx::model::HtxRawResponse, models::normalized::NormalizedResponse};

pub fn normalize_htx_response(raw: HtxRawResponse) -> Vec<NormalizedResponse> {
    let symbol = raw
        .ch
        .replace("market.", "")
        .replace(".trade.detail", "")
        .to_uppercase();

    raw.tick
        .data
        .into_iter()
        .map(|trade| NormalizedResponse {
            exchange: "htx".to_string(),
            symbol: symbol.clone(),
            event_type: "trade".to_string(),
            event_time: trade.ts.to_string(),
            trade_id: trade.trade_id.to_string(),
            last_price: trade.price.to_string(),
            quantity: trade.amount.to_string(),
            is_buyer_maker: Some(trade.direction == "sell"),
            timestamp: trade.ts,
        })
        .collect()
}
