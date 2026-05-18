use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::bitfinex::{
        model::{BitfinexRawResponse, SubMessageBitfinex},
        normalize::normalize_bitfinex_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct BitfinexAdapter;

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
        if text.starts_with('{') {
            return vec![];
        }

        let parsed = serde_json::from_str::<BitfinexRawResponse>(text);
        match parsed {
            Ok(payload) => normalize_bitfinex_response(payload),
            Err(_) => vec![]
        }
    }
}
