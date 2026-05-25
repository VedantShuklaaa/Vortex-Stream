use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::bybit::{
        model::{BybitRawResponse, SubMessageBybit},
        normalize::normalize_bybit_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct BybitAdapter;

impl ExchangeAdapter for BybitAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://stream.bybit.com/v5/public/spot"
    }

    fn default_symbols(&self) -> Vec<String> {
        vec![
            "BTCUSDT".to_string(),
            "ETHUSDT".to_string(),
            "SOLUSDT".to_string(),
        ]
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        symbol.to_string()
    }

	fn subscribe_message(&self, symbol: &str) -> Result<String> {
		let payload = SubMessageBybit {
			op: "subscribe".to_string(),
			args: vec![format!("publicTrade.{}", symbol)]
		};

		Ok(serde_json::to_string(&payload)?)
	}

	fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
		let payload = SubMessageBybit {
			op: "unsubscribe".to_string(),
			args: vec![format!("publicTrade.{}", symbol)]
		};

		Ok(serde_json::to_string(&payload)?)
	}

	fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
		let parsed = serde_json::from_str::<BybitRawResponse>(text);
		match parsed {
			Ok(payload) => {
				normalize_bybit_response(payload)
			}
			Err(_) => vec![]
		}
	}
}
