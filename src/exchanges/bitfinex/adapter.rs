use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::bitfinex::{
        model::{BitfinexRawResponse, SubMessageBitfinex},
        normalize::normalize_bitfinex_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct BitfinexAdapter {
    channels: Arc<Mutex<HashMap<u64, String>>>,
}

impl BitfinexAdapter {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl ExchangeAdapter for BitfinexAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://api-pub.bitfinex.com/ws/2"
    }

    fn default_symbols(&self) -> Vec<String> {
        vec![
            "tBTCUSD".to_string(),
            "tETHUSD".to_string(),
            "tSOLUSD".to_string(),
        ]
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        if symbol.ends_with("USDT") {
            let base = symbol.trim_end_matches("USDT");
            format!("t{}USD", base)
        } else {
            symbol.to_string()
        }
    }

    fn subscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageBitfinex {
            event: "subscribe".to_string(),
            channel: "trades".to_string(),
            symbol: format!("{}", symbol.to_string()),
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageBitfinex {
            event: "unsubscribe".to_string(),
            channel: "trades".to_string(),
            symbol: format!("t{}", symbol.to_string()),
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
        println!("raw: {}", text);
        if text.starts_with('{') {
            let value: serde_json::Value = match serde_json::from_str(text) {
                Ok(v) => v,
                Err(_) => return vec![],
            };

            if value["event"] == "subscribed" {
                let chan_id = value["chanId"].as_u64().unwrap();

                let symbol = value["symbol"]
                    .as_str()
                    .unwrap()
                    .replace("t", "")
                    .replace("USD", "USDT");

                self.channels
                    .lock()
                    .unwrap()
                    .insert(chan_id, symbol.clone());

                println!("[bitfinex] mapped {} -> {}", chan_id, symbol);
            }

            return vec![];
        }

        let parsed = serde_json::from_str::<BitfinexRawResponse>(text);
        match parsed {
            Ok(payload) => {
                println!("parsed successfully");
                let channel_id = match &payload {
                    BitfinexRawResponse::Snapshot(channel_id, _) => *channel_id,
                    BitfinexRawResponse::Update(channel_id, _, _) => *channel_id,
                };

                let symbol = self
                    .channels
                    .lock()
                    .unwrap()
                    .get(&channel_id)
                    .cloned()
                    .unwrap_or("UNKNOWN".to_string());

                normalize_bitfinex_response(payload, symbol)
            }
            Err(_) => vec![],
        }
    }
}
