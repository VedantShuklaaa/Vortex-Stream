use crate::models::normalized::NormalizedResponse;
use std::sync::Arc;

pub type TradeCallback = Arc<dyn Fn(NormalizedResponse) + Send + Sync>;

//
// exchange selector enum
//
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Exchange {
    Binance,
    Coinbase,
    Okx,
    Bybit,
    Kraken,
    Bitget,
    Bitfinex,
    CryptoCom,
    Htx,
    Bitstamp,
}

#[derive(Debug, Clone)]
pub enum ExchangeCommand {
    Subscribe(String),
    Unsubscribe(String),
}
