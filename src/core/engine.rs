use anyhow::Result;
use flate2::read::GzDecoder;
use futures_util::{SinkExt, StreamExt};
use std::io::Read;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::{broadcast::Sender, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{
    core::{adapter::ExchangeAdapter, types::ExchangeCommand},
    models::normalized::NormalizedResponse,
};

pub async fn start_engine<A>(
    adapter: A,
    tx: Arc<Sender<NormalizedResponse>>,
    mut cmd_rx: mpsc::Receiver<ExchangeCommand>,
) -> Result<()>
where
    A: ExchangeAdapter,
{
    //
    // runtime subscription state
    //
    let mut active_symbols = HashSet::<String>::new();

    //
    // default symbols
    //
    for symbol in adapter.default_symbols() {
        active_symbols.insert(symbol);
    }

    loop {
        let url = adapter.websocket_url();
        println!("connecting to {}", url);

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

        //
        // restore subscriptions
        //
        for symbol in &active_symbols {
            let normalized = adapter.normalize_symbol(symbol);
            let payload = adapter.subscribe_message(&normalized)?;

            if let Err(err) = write.send(Message::Text(payload.into())).await {
                eprintln!("failed to subscribe: {}", err);
            }
        }
        println!("subscriptions restored");

        loop {
            tokio::select! {
                Some(message) = read.next() => {
                    match message {
                        Ok(Message::Text(text)) => {
                            let parsed = adapter.parse_message(&text);

                            if parsed.is_empty() {
                                continue
                            }
                            for trade in parsed {
                                if let Err(err) = tx.send(trade.clone()) {
                                eprintln!("broadcast failed: {}", err);
                            } else {
                                println!("broadcasted: {:?}", trade);
                            }}
                        }

                        Ok(Message::Binary(bin)) => {
                            let mut decoder = GzDecoder::new(&bin[..]);
                            let mut text = String::new();

                            decoder
                                .read_to_string(&mut text)
                                .unwrap();

                            let parsed = adapter.parse_message(&text);

                            if text.contains("\"ping\"") {
                                let pong = text.replace("ping","pong");
                                let _ = write.send(Message::Text(pong.into())).await;

                                continue;
                            }

                            for trade in parsed {
                                let _ = tx.send(trade);
                            }
                        }

                        Ok(Message::Close(_)) => {println!("websocket closed");
                            break;
                        }

                        Ok(_) => {}

                        Err(err) => {
                            eprintln!("websocket error: {}", err);
                            break;
                        }
                    }
                }



                //
                // runtime commands
                //
                Some(command) = cmd_rx.recv() => {
                    match command {
                        ExchangeCommand::Subscribe(symbol) => {
                            if active_symbols.contains(&symbol) {
                                continue;
                            }

                            active_symbols.insert(
                                symbol.clone()
                            );

                            let normalized = adapter.normalize_symbol(&symbol);
                            let payload = adapter.subscribe_message(&normalized)?;

                            if let Err(err) = write.send(Message::Text(payload.into())).await {
                                eprintln!("subscribe error: {}", err);
                            }

                            println!("subscribed: {}",symbol);
                        }

                        ExchangeCommand::Unsubscribe(symbol) => {
                            active_symbols.remove(&symbol);
                            let payload = adapter.unsubscribe_message(&symbol)?;

                            if let Err(err) =write.send(Message::Text(payload.into())).await {
                                eprintln!("unsubscribe error: {}", err);
                            }

                            println!("unsubscribed: {}", symbol);
                        }
                    }
                }
            }
        }

        println!("reconnecting in 5 seconds");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
