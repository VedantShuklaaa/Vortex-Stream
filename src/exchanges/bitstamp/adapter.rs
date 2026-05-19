use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::bitstamp::{
        model::{BitstampChannelData, BitstampRawResponse, SubMessageBitstamp},
        normalize::normalize_bitstamp_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct BitstampAdapter;

impl ExchangeAdapter for BitstampAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://ws.bitstamp.net"
    }

    fn default_symbols(&self) -> Vec<String> {
        vec![
            "BTCUSDT".to_string(),
            "ETHUSDT".to_string(),
            "SOLUSDT".to_string(),
        ]
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        if symbol.ends_with("USDT") {
            let base = symbol.trim_end_matches("USDT");
            format!("{}usd", base.to_lowercase())
        } else {
            symbol.to_lowercase()
        }
    }

    fn subscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageBitstamp {
            event: "bts:subscribe".to_string(),
            data: BitstampChannelData {
                channel: format!("live_trades_{}", symbol),
            },
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageBitstamp {
            event: "bts:unsubscribe".to_string(),
            data: BitstampChannelData {
                channel: format!("live_trades_{}", symbol),
            },
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
        if !text.contains("\"event\":\"trade\"") && !text.contains("\"event\": \"trade\"") {
            return vec![];
        }
        let parsed = serde_json::from_str::<BitstampRawResponse>(text);
        match parsed {
            Ok(payload) => normalize_bitstamp_response(payload),
            Err(_) => vec![]
        }
    }
}
