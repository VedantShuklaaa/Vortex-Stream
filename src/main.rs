mod exchanges;
mod models;

use anyhow::Result;
use rustls::crypto::ring;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::{exchanges::binance::binance::binance_stream, models::model::NormalizedResponse};

/*fn spawn_symbol_listener(
    symbol: String,
    tx: Arc<broadcast::Sender<NormalizedResponse>>,
) -> tokio::task::JoinHandle<()> {
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(data) = rx.recv().await {
            if data.symbol != symbol {
                continue;
            }

            println!("{} subscriber: {:?}", symbol, data);
        }
    })
}*/

struct SubscriptionManager {
    tx: Arc<broadcast::Sender<NormalizedResponse>>,
}

impl SubscriptionManager {
    fn new(tx: Arc<broadcast::Sender<NormalizedResponse>>) -> Self {
        Self { tx }
    }

    fn subscribe(&self, symbol: String) -> tokio::task::JoinHandle<()> {
        let mut rx = self.tx.subscribe();

        tokio::spawn(async move {
            while let Ok(data) = rx.recv().await {
                if data.symbol != symbol {
                    continue;
                }

                println!("{} subscriber: {:?}", symbol, data);
            }
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    ring::default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("rustls crypto provider already installed"))?;

    let (tx, _) = broadcast::channel::<NormalizedResponse>(100);

    let tx = Arc::new(tx);

    // websocket task
    let ws_tx = tx.clone();

    let ws_task = tokio::spawn(async move {
        if let Err(e) = binance_stream(ws_tx).await {
            eprintln!("binance stream error: {e}");
        }
    });

    
    //manager task
    let manager = SubscriptionManager::new(tx.clone());

    let btc_task = manager.subscribe("BTCUSDT".to_string());

    // keep runtime alive naturally
    let _ = tokio::join!(ws_task, btc_task,);

    Ok(())
}
