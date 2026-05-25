use crate::{
    exchanges::bitfinex::model::{BitfinexRawResponse, BitfinexTrade},
    models::normalized::NormalizedResponse,
};

pub fn normalize_bitfinex_response(
    raw: BitfinexRawResponse,
    symbol: String,
) -> Vec<NormalizedResponse> {
    match raw {
        // snapshot
        BitfinexRawResponse::Snapshot(_channel_id, trades) => trades
            .into_iter()
            .map(|trade| normalize_trade(trade, symbol.clone()))
            .collect(),

        // live updates
        BitfinexRawResponse::Update(_channel_id, event_type, trade) => {
            // ignore execution events if needed
            if event_type != "tu" && event_type != "te" {
                return vec![];
            }

            vec![normalize_trade(trade, symbol)]
        }
    }
}

fn normalize_trade(trade: BitfinexTrade, symbol: String) -> NormalizedResponse {
    let (trade_id, timestamp, amount, price) = trade;

    NormalizedResponse {
        exchange: "bitfinex".to_string(),

        symbol,

        event_type: "trade".to_string(),

        event_time: timestamp.to_string(),

        trade_id: trade_id.to_string(),

        last_price: price.to_string(),

        quantity: amount.abs().to_string(),

        is_buyer_maker: Some(amount < 0.0),

        timestamp,
    }
}
