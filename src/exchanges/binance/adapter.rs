use anyhow::{Ok, Result};

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::binance::{
        model::{BinanceRawResponse, SubMessageBinance},
        normalize::normalize_binance_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct BinanceAdapter;

impl ExchangeAdapter for BinanceAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://data-stream.binance.vision:443/ws"
    }

    fn default_symbols(&self) -> Vec<String> {
        vec![
            "btcusdt@trade".to_string(),
            "ethusdt@trade".to_string(),
            "solusdt@trade".to_string(),
        ]
    }

    fn subscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageBinance {
            method: "SUBSCRIBE".to_string(),
            params: vec![format!("{}@trade", symbol.to_lowercase())],
            id: 1,
        };

        Ok(serde_json::to_string(&payload)?)
    }

	fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
		let payload = SubMessageBinance {
            method: "UNSUBSCRIBE".to_string(),
            params: vec![format!("{}@trade", symbol.to_lowercase())],
            id: 1,
        };

		Ok(serde_json::to_string(&payload)?)
	}

    fn parse_message(&self, text: &str) -> Option<NormalizedResponse> {
        if !text.contains("\"e\"") {
            return None;
        }
        let parsed = serde_json::from_str::<BinanceRawResponse>(text).ok()?;

		Some(normalize_binance_response(parsed))
    }
}
