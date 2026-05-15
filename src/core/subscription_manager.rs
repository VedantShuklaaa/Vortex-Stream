use std::sync::Arc;
use tokio::sync::broadcast;
use crate::{
	models::normalized::{NormalizedResponse},
	core::types::TradeCallback
};

pub struct SubscriptionManager {
    tx: Arc<broadcast::Sender<NormalizedResponse>>,
}

impl SubscriptionManager {
    pub fn new(tx: Arc<broadcast::Sender<NormalizedResponse>>) -> Self {
        Self { tx }
    }

    pub fn subscribe(&self, symbol: String, callback: TradeCallback) -> tokio::task::JoinHandle<()> {
        let mut rx = self.tx.subscribe();

        tokio::spawn(async move {
            while let Ok(data) = rx.recv().await {
                if data.symbol != symbol {
                    continue;
                }

                callback(data);
            }
        })
    }
}