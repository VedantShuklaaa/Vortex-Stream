use crate::{
    core::{
        engine::start_engine,
        subscription_manager::SubscriptionManager,
        types::{Exchange, ExchangeCommand, TradeCallback},
    },
    exchanges::{
        binance::adapter::BinanceAdapter, bybit::adapter::BybitAdapter, coinbase::adapter::CoinbaseAdapter, kraken::adapter::KrakenAdapter, okx::adapter::OkxAdapter
    },
    models::normalized::NormalizedResponse,
};

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, mpsc};

/// Main SDK entrypoint for realtime
/// market data streaming.
pub struct VortexStream {
    pub manager: SubscriptionManager,
    pub command_senders: HashMap<Exchange, mpsc::Sender<ExchangeCommand>>,
    pub command_receivers: HashMap<Exchange, mpsc::Receiver<ExchangeCommand>>,
    pub exchanges: Vec<Exchange>,
    pub started: bool,
}

impl VortexStream {
    /// Creates a new realtime stream
    /// engine instance.
    pub fn new(exchanges: Vec<Exchange>) -> Self {
        let (tx, _) = broadcast::channel::<NormalizedResponse>(100);
        let tx = Arc::new(tx);
        let mut command_senders = HashMap::new();
        let mut command_receivers = HashMap::new();

        //
        // create runtime channels only
        //
        for exchange in &exchanges {
            let (cmd_tx, cmd_rx) = mpsc::channel::<ExchangeCommand>(100);
            command_senders.insert(exchange.clone(), cmd_tx);
            command_receivers.insert(exchange.clone(), cmd_rx);
        }

        let manager = SubscriptionManager::new(tx);

        Self {
            manager,
            command_senders,
            command_receivers,
            exchanges,
            started: false,
        }
    }

    //
    // runtime startup
    //
    pub async fn start(&mut self) {
        if self.started {
            return;
        }
        self.started = true;
        for exchange in &self.exchanges {
            let cmd_rx = self.command_receivers.remove(exchange).unwrap();

            //
            // replace sender
            //
            match exchange {
                Exchange::Binance => {
                    let engine_tx = self.manager.tx.clone();

                    tokio::spawn(async move {
                        if let Err(err) = start_engine(BinanceAdapter, engine_tx, cmd_rx).await {
                            eprintln!("binance engine error: {}", err);
                        }
                    });
                }

                Exchange::Coinbase => {
                    let engine_tx = self.manager.tx.clone();

                    tokio::spawn(async move {
                        if let Err(err) = start_engine(CoinbaseAdapter, engine_tx, cmd_rx).await {
                            eprintln!("coinbase engine error: {}", err);
                        }
                    });
                }

                Exchange::Okx => {
                    let engine_tx = self.manager.tx.clone();

                    tokio::spawn(async move {
                        if let Err(err) = start_engine(OkxAdapter, engine_tx, cmd_rx).await {
                            eprintln!("okx engine error: {}", err);
                        }
                    });
                }

                Exchange::Bybit => {
                    let engine_tx = self.manager.tx.clone();

                    tokio::spawn(async move {
                        if let Err(err) = start_engine(BybitAdapter, engine_tx, cmd_rx).await {
                            eprintln!("bybit engine error: {}", err);
                        }
                    });
                }

                Exchange::Kraken => {
                    let engine_tx = self.manager.tx.clone();

                    tokio::spawn(async move {
                        if let Err(err) = start_engine(KrakenAdapter, engine_tx, cmd_rx).await {
                            eprintln!("kraken engine error: {}", err);
                        }
                    });
                }
            }
        }
    }

    //
    // runtime unsubscribe api
    //
    pub async fn unsubscribe(&self, exchange: Exchange, symbol: &str) {
        if let Some(sender) = self.command_senders.get(&exchange) {
            let _ = sender
                .send(ExchangeCommand::Unsubscribe(symbol.to_string()))
                .await;
        }
    }

    //
    // trade callback api
    //
    pub async fn trades<F>(
        &self,
        exchange: Exchange,
        symbol: &str,
        callback: F,
    ) -> tokio::task::JoinHandle<()>
    where
        F: Fn(NormalizedResponse) + Send + Sync + 'static,
    {
        //
        // send runtime subscribe
        //
        if let Some(sender) = self.command_senders.get(&exchange) {
            let _ = sender
                .send(ExchangeCommand::Subscribe(symbol.to_string()))
                .await;
        }

        let callback: TradeCallback = Arc::new(callback);
        let normalized_symbol = match exchange {
            Exchange::Okx => {
                if symbol.ends_with("USDT") {
                    let base = symbol.trim_end_matches("USDT");
                    format!("{}-USDT", base)
                } else {
                    symbol.to_string()
                }
            }

            Exchange::Coinbase => {
                if symbol.ends_with("USDT") {
                    let base = symbol.trim_end_matches("USDT");
                    format!("{}-USD", base)
                } else {
                    symbol.to_string()
                }
            }
            _ => symbol.to_string(),
        };

        //
        // delegate to manager
        //
        self.manager.subscribe(normalized_symbol, callback)
    }
}
