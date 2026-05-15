use std::sync::Arc;
use crate::{
	models::normalized::{NormalizedResponse},
};

pub type TradeCallback = Arc<dyn Fn(NormalizedResponse) + Send + Sync>;