use std::sync::Arc;
use crate::{
	models::normalized::{NormalizedResponse},
};

pub type TradeCallback = Arc<dyn Fn(NormalizedResponse) + Send + Sync>;

//
// exchange selector enum
//
#[derive(Debug, Clone)]
pub enum Exchange {
    Binance,
    Coinbase,
}