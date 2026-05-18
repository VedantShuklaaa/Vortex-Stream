use crate::{exchanges::kraken::model::KrakenRawResponse, models::normalized::NormalizedResponse};

pub fn normalize_kraken_response(raw: KrakenRawResponse) -> Vec<NormalizedResponse> {
    let (_channel_id, trades, _channel_name, symbol) = raw;

    trades
        .into_iter()
        .map(|trade| {
            let (price, volume, timestamp, side, _order_type, _misc) = trade;

            NormalizedResponse {
                exchange: "kraken".to_string(),
                symbol: symbol.replace("XBT", "BTC").replace("/USD", "USDT"),
                event_type: "trade".to_string(),
                event_time: timestamp.clone(),
                trade_id: "".to_string(),
                last_price: price,
                quantity: volume,
                is_buyer_maker: Some(side == "s"),
                timestamp: (timestamp.parse::<f64>().unwrap() * 1000.0) as u64,
            }
        })
        .collect()
}
