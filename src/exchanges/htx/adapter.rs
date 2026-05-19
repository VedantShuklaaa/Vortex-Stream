use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::htx::{
        model::{HtxRawResponse, SubMessageHtx, UnSubMessageHtx},
        normalize::normalize_htx_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct HtxAdapter;

impl ExchangeAdapter for HtxAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://api.huobi.pro/ws"
    }

    fn default_symbols(&self) -> Vec<String> {
        vec![
            "btcusdt".to_string(),
            "ethusdt".to_string(),
            "solusdt".to_string(),
        ]
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        symbol.to_lowercase()
    }

    fn subscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageHtx {
            sub: format!("market.{}.trade.detail", symbol),
            id: "1".to_string(),
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = UnSubMessageHtx {
            unsub: format!("market.{}.trade.detail", symbol),
            id: "1".to_string(),
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
        if text.contains("\"ping\"") || text.contains("\"pong\"") {
            return vec![];
        }

        if !text.contains("\"ch\"") {
			return vec![];
		}
        let parsed = serde_json::from_str::<HtxRawResponse>(text);

        match parsed {
            Ok(payload) => normalize_htx_response(payload),
            Err(_) =>  vec![]
        }
    }
}
