use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::kraken::{
        model::{KrakenRawResponse, KrakenSubscription, SubMessageKraken},
        normalize::normalize_kraken_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct KrakenAdapter;

impl ExchangeAdapter for KrakenAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://ws.kraken.com"
    }

    fn default_symbols(&self) -> Vec<String> {
        vec![
            "BTC/USD".to_string(),
            "ETH/USD".to_string(),
            "SOL/USD".to_string(),
        ]
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        if symbol.ends_with("USDT") {
            let base = symbol.trim_end_matches("USDT");

            format!("{}/USD", base)
        } else {
            symbol.to_string()
        }
    }

    fn subscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageKraken {
            event: "subscribe".to_string(),
            subscription: KrakenSubscription {
                name: "trade".to_string(),
            },
            pair: vec![symbol.to_string()],
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageKraken {
            event: "unsubscribe".to_string(),
            subscription: KrakenSubscription {
                name: "trade".to_string(),
            },
            pair: vec![symbol.to_string()],
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
        if text.contains("heartbeat") {
            return vec![];
        }

        if text.starts_with('{') {
            return vec![];
        }

        let parsed = serde_json::from_str::<KrakenRawResponse>(text);
        match parsed {
            Ok(payload) => normalize_kraken_response(payload),
            Err(_) => vec![],
        }
    }
}
