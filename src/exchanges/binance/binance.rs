use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{
    exchanges::binance::model::BinanceRawResponse,
    models::model::{NormalizedResponse, SubscribeMessage},
};

fn normalize_binance_response(raw: BinanceRawResponse) -> NormalizedResponse {
    NormalizedResponse {
        exchange: "binance".to_string(),
        symbol: raw.symbol,
        last_price: raw.last_price,
        quantity: raw.last_quantity,
        timestamp: raw.timestamp,
    }
}

pub async fn binance_stream(tx: Arc<Sender<NormalizedResponse>>) -> Result<()> {
    let url = "wss://data-stream.binance.vision:443/ws";

    println!("connecting to {url}");
    let (ws_stream, _) = connect_async(url).await?;
    println!("connected to Binance");

    let (mut write, mut read) = ws_stream.split();

    let subscribe_msg = SubscribeMessage {
        method: "SUBSCRIBE".to_string(),
        params: vec![
            "btcusdt@trade".to_string(),
            "ethusdt@trade".to_string(),
            "solusdt@trade".to_string(),
        ],
        id: 1,
    };

	let json_message = serde_json::to_string(&subscribe_msg)?;
    write.send(Message::Text(json_message.into())).await?;

    while let Some(message) = read.next().await {
        let message = message?;

        if message.is_text() {
            let text = message.to_text()?;
            let raw: BinanceRawResponse = match serde_json::from_str(text) {
                Ok(r) => r,
                Err(_) => {
                    println!("skiping non ticker message: {text}");
                    continue;
                }
            };
            let normalized_response = normalize_binance_response(raw);

            let _ = tx.send(normalized_response);
        }
    }

    Ok(())
}
