use crate::{
    core::{
        subscription_manager::SubscriptionManager,
        types::{Exchange, TradeCallback},
    },
    exchanges::{binance::stream::binance_stream, coinbase::stream::coinbase_stream},
    models::normalized::NormalizedResponse,
};
use std::sync::Arc;
use tokio::sync::broadcast;

/// Main SDK entrypoint for realtime
/// market data streaming.
pub struct VortexStream {
    pub manager: SubscriptionManager,
}

impl VortexStream {
    /// Creates a new realtime stream
    /// engine instance.
    pub fn new(exchanges: Vec<Exchange>) -> Self {
        let (tx, _) = broadcast::channel::<NormalizedResponse>(100);
        let tx = Arc::new(tx);

        for exchange in exchanges {
            match exchange {
                Exchange::Binance => {
                    let binance_tx = tx.clone();

                    tokio::spawn(async move {
                        if let Err(err) = binance_stream(binance_tx).await {
                            eprintln!("binance stream error: {}", err);
                        }
                    });
                }

                Exchange::Coinbase => {
                    let coinbase_tx = tx.clone();

                    tokio::spawn(async move {
                        if let Err(err) = coinbase_stream(coinbase_tx).await {
                            eprintln!("coinbase stream error: {}", err);
                        }
                    });
                }
            }
        }

        let manager = SubscriptionManager::new(tx.clone());

        Self { manager }
    }

    /// Subscribe to realtime trade
    /// events for a symbol.
    pub fn trades<F>(&self, symbol: &str, callback: F) -> tokio::task::JoinHandle<()>
    where
        F: Fn(NormalizedResponse) + Send + Sync + 'static,
    {
        let callback: TradeCallback = Arc::new(callback);

        //
        // delegate to manager
        //
        self.manager.subscribe(symbol.to_string(), callback)
    }
}
