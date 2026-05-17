use anyhow::Result;

use crate::models::normalized::NormalizedResponse;

pub trait ExchangeAdapter {
    //
    // websocket endpoint
    //
    fn websocket_url(&self) -> &'static str;

	//
	// default symbols
	//
    fn default_symbols(&self) -> Vec<String>;

    fn normalize_symbol(
        &self,
        symbol: &str,
    ) -> String;
    
    //
    // build subscribe payload
    //
    fn subscribe_message(&self, symbol: &str) -> Result<String>;

    //
    // build unsubscribe payload
    //
    fn unsubscribe_message(&self, symbol: &str) -> Result<String>;

    //
    // parse + normalize exchange payload
    //
    fn parse_message(&self, text: &str) -> Option<NormalizedResponse>;
}
