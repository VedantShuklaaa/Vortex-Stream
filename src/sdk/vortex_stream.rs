use crate::{
    core::{subscription_manager::SubscriptionManager, types::TradeCallback},
    exchanges::binance::stream::binance_stream,
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
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<NormalizedResponse>(100);
        let tx = Arc::new(tx);

        let ws_tx = tx.clone();

        let _ = tokio::spawn(async move {
            if let Err(err) = binance_stream(ws_tx).await {
                eprint!("binance stream error: {}", err);
            }
        });

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
