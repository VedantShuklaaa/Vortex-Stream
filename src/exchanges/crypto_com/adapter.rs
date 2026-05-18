use anyhow::Result;

use crate::{
    core::adapter::ExchangeAdapter,
    exchanges::crypto_com::{
        model::{CryptoComParams, CryptoComRawResponse, SubMessageCryptoCom},
        normalize::normalize_crypto_com_response,
    },
    models::normalized::NormalizedResponse,
};

pub struct CryptoComAdapter;

impl ExchangeAdapter for CryptoComAdapter {
    fn websocket_url(&self) -> &'static str {
        "wss://stream.crypto.com/exchange/v1/market"
    }

    fn default_symbols(&self) -> Vec<String> {
        vec![
            "BTC_USDT".to_string(),
            "ETH_USDT".to_string(),
            "SOL_USDT".to_string(),
        ]
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        if symbol.ends_with("USDT") {
            let base = symbol.trim_end_matches("USDT");
            format!("{}_USDT", base)
        } else {
            symbol.to_string()
        }
    }

    fn subscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageCryptoCom {
            id: 1,
            method: "subscribe".to_string(),
            params: CryptoComParams {
                channels: vec![format!("trade.{}", symbol)],
            },
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn unsubscribe_message(&self, symbol: &str) -> Result<String> {
        let payload = SubMessageCryptoCom {
            id: 1,
            method: "unsubscribe".to_string(),
            params: CryptoComParams {
                channels: vec![format!("trade.{}", symbol)],
            },
        };

        Ok(serde_json::to_string(&payload)?)
    }

    fn parse_message(&self, text: &str) -> Vec<NormalizedResponse> {
        if !text.contains("\"data\"") {
            return vec![];
        }

        let parsed = serde_json::from_str::<CryptoComRawResponse>(text);
        match parsed {
            Ok(payload) => normalize_crypto_com_response(payload),
            Err(err) => {
                println!("crypto.com parse error: {}", err);
                vec![]
            }
        }
    }
}
