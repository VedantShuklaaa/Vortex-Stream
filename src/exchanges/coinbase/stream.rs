use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{
    exchanges::coinbase::{
        model::{CoinbaseRawResponse, SubMessageCoinbase},
        normalize::normalize_coinbase_response,
    },
    models::normalized::NormalizedResponse,
};

pub async fn coinbase_stream(tx: Arc<Sender<NormalizedResponse>>) -> Result<()> {
    let url = "wss://ws-feed.exchange.coinbase.com";

    loop {
        println!("connecting to url {url}");
        let connection = connect_async(url).await;
        let (ws_stream, _) = match connection {
            Ok(success) => {
                println!("connected successfully");
                success
            }
            Err(err) => {
                eprintln!("connection failed: {}", err);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        let (mut write, mut read) = ws_stream.split();

        let subscribe_msg = SubMessageCoinbase {
            r#type: "subscribe".to_string(),
            product_ids: vec![
                "BTC-USD".to_string(),
                "ETH-USD".to_string(),
                "SOL-USD".to_string(),
            ],
            channels: vec!["ticker".to_string()],
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
                    if !text.contains("\"type\":\"ticker\"") {
                        continue;
                    }

                    let parsed = serde_json::from_str::<CoinbaseRawResponse>(&text);
                    match parsed {
                        Ok(wrapper) => {
                            let normalized_response = normalize_coinbase_response(wrapper);
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
