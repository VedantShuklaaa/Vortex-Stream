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

    loop {
        println!("connecting to {url}");
        let connection = connect_async(url).await;
        let (ws_stream, _) = match connection {
            Ok(success) => {
                println!("connected successfully");
                success
            }
            Err(err) => {
                eprint!("connection failed: {}", err);

                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                continue;
            }
        };

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
        if let Err(err) = write.send(Message::Text(json_message.into())).await {
            eprintln!("failed to subscribe: {}", err);
            continue;
        }
        println!("subscribed successfully");

        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let parsed = serde_json::from_str::<BinanceRawResponse>(&text);

                    match parsed {
                        Ok(wrapper) => {
                            let normalized_response = normalize_binance_response(wrapper);

                            let _ = tx.send(normalized_response);
                        }

                        Err(err) => {
                            eprintln!("parse error: {}", err);
                        }
                    }
                }

                Ok(Message::Close(_)) => {
                    println!("websocket closed. reconnecting...");

                    break;
                }

                Ok(_) => {}

                Err(err) => {
                    eprintln!("websocket error: {}", err);

                    break;
                }
            }
        }

        println!("reconnecting in 5 seconds");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
