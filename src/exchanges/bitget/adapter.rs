use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::bitget::{
        model::{BitgetRawResponse, SubMessageBitget, Args},
        normalize::normalize_bitget_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct BitgetAdapter;

impl ExchangeAdapter for BitgetAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://ws.bitget.com/v2/ws/public"
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
		let payload = SubMessageBitget {
			op: "subscribe".to_string(),
			args: vec![Args {
				inst_type: "SPOT".to_string(),
				channel: "trade".to_string(),
				inst_id: symbol.to_string(),	
			}]
		};

		Ok(serde_json::to_string(&payload)?)
	}

	fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
		let payload = SubMessageBitget {
			op: "unsubscribe".to_string(),
			args: vec![Args {
				inst_type: "SPOT".to_string(),
				channel: "trade".to_string(),
				inst_id: symbol.to_string(),	
			}]
		};

		Ok(serde_json::to_string(&payload)?)
	}

	fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
		if text.contains("\"event\"") {
			return vec![];
		}

		let parsed = serde_json::from_str::<BitgetRawResponse>(text);
		match parsed {
			Ok(payload) => {
				normalize_bitget_response(payload)
			}
			Err(_) => vec![]
		}
	}
}
