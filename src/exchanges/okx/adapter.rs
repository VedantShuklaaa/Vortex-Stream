use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::okx::{
        model::{OkxRawResponse, OkxSubscriptionArg, SubMessageOkx},
        normalize::normalize_okx_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct OkxAdapter;

impl ExchangeAdapter for OkxAdapter {
    fn websocket_url(&self) -> &'static str {
        println!("connecting to okx");
        "wss://ws.okx.com:8443/ws/v5/public"
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

            format!("{}-USDT", base)
        } else {
            symbol.to_string()
        }
    }

    fn subscribe_message(&self, symbol: &str) -> Result<String> {
        let symbol = self.normalize_symbol(symbol);

        let payload = SubMessageOkx {
            op: "subscribe".to_string(),
            args: vec![OkxSubscriptionArg {
                channel: "trades".to_string(),
                inst_id: symbol,
            }],
        };

        println!("{}", serde_json::to_string(&payload)?);
        Ok(serde_json::to_string(&payload)?)
    }

    fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
        let symbol = self.normalize_symbol(symbol);

        let payload = SubMessageOkx {
            op: "unsubscribe".to_string(),
            args: vec![OkxSubscriptionArg {
                channel: "trades".to_string(),
                inst_id: symbol,
            }],
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
        let parsed = serde_json::from_str::<OkxRawResponse>(text);
        match parsed {
            Ok(payload) => {
                if payload.data.is_empty() {
                    return vec![];
                }
                let normalized = normalize_okx_response(payload);
                vec![normalized]
            }

            Err(err) => {
                println!("parse error: {}", err);
                vec![]
            }
        }
    }
}
