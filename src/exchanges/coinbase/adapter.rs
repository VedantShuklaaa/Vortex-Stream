use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::coinbase::{
        model::{CoinbaseRawResponse, SubMessageCoinbase},
        normalize::normalize_coinbase_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct CoinbaseAdapter;

impl ExchangeAdapter for CoinbaseAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://ws-feed.exchange.coinbase.com"
    }

    fn default_symbols(&self) -> Vec<String> {
        vec![
            "BTC-USD".to_string(),
            "ETH-USD".to_string(),
            "SOL-USD".to_string(),
        ]
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        if symbol.ends_with("USDT") {
            let base = symbol.trim_end_matches("USDT");

            format!("{}-USD", base)
        } else {
            symbol.to_string()
        }
    }

    fn subscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageCoinbase {
            r#type: "subscribe".to_string(),
            product_ids: vec![symbol.to_string()],
            channels: vec!["ticker".to_string()],
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageCoinbase {
            r#type: "unsubscribe".to_string(),
            product_ids: vec![symbol.to_string()],
            channels: vec!["ticker".to_string()],
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
        let value: serde_json::Value = match serde_json::from_str(text) {
            Ok(v) => v,
            Err(err) => {
                println!("json parse error: {}", err);
                return vec![];
            }
        };

        if value["type"] != "ticker" {
            return vec![];
        }

        let parsed = serde_json::from_str::<CoinbaseRawResponse>(text);

        match parsed {
            Ok(payload) => {
                let normalized = normalize_coinbase_response(payload);
                vec![normalized]
            }
            Err(err) => {
                println!("parse error: {:?}", err);
                vec![]
            }
        }
    }
}
