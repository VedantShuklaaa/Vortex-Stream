use crate::{exchanges::bitget::model::BitgetRawResponse, models::normalized::NormalizedResponse};

pub fn normalize_bitget_response(raw: BitgetRawResponse) -> Vec<NormalizedResponse> {
    raw.data
        .into_iter()
        .map(|trade| NormalizedResponse {
            exchange: "bitget".to_string(),
            symbol: raw.arg.inst_id.replace("-", ""),
            event_type: "trade".to_string(),
            event_time: trade.ts.to_string(),
            trade_id: trade.trade_id,
            last_price: trade.price,
            quantity: trade.size,
            is_buyer_maker: Some(trade.side == "sell"),
            timestamp: trade.ts.parse().unwrap(),
        })
        .collect()
}
