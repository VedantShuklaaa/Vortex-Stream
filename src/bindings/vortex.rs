use napi::{bindgen_prelude::Function, threadsafe_function::ThreadsafeFunctionCallMode};
use napi_derive::napi;
use std::sync::Arc;
use tokio::{runtime::Runtime, sync::Mutex};

use crate::{Exchange, VortexStream};

#[napi]
pub struct JsVortexStream {
    inner: Arc<Mutex<VortexStream>>,
    runtime: Arc<Runtime>,
}

#[napi]
impl JsVortexStream {
    #[napi(constructor)]
    pub fn new() -> Self {
        //
        // create runtime
        //
        let runtime = Runtime::new().unwrap();

        //
        // enter runtime context
        //
        let _guard = runtime.enter();

        //
        // create vortex engine
        //
        let stream = VortexStream::new(vec![
            Exchange::Binance,
            Exchange::Coinbase,
            Exchange::Okx,
            Exchange::Bybit,
            Exchange::Kraken,
            Exchange::Bitget,
            Exchange::Bitfinex,
            Exchange::CryptoCom,
            Exchange::Htx,
            Exchange::Bitstamp,
        ]);
        let inner = Arc::new(Mutex::new(stream));

        //
        // start engines ONCE
        //
        let inner_clone = inner.clone();

        runtime.spawn(async move {
            let mut locked = inner_clone.lock().await;
            locked.start().await;
        });

        Self {
            inner,
            runtime: Arc::new(runtime),
        }
    }

    #[napi]
    pub fn trades(
        &self,
        exchange: String,
        symbol: String,
        callback: Function<'_, String, ()>,
    ) -> napi::Result<()> {
        //
        // exchange parser
        //
        let exchange = match exchange.as_str() {
            "binance" => Exchange::Binance,
            "coinbase" => Exchange::Coinbase,
            "okx" => Exchange::Okx,
            "bybit" => Exchange::Bybit,
            "kraken" => Exchange::Kraken,
            "bitget" => Exchange::Bitget,
            "bitfinex" => Exchange::Bitfinex,
            "crypto_com" => Exchange::CryptoCom,
            "htx" => Exchange::Htx,
            "bitstamp" => Exchange::Bitstamp,

            _ => {
                println!("unsupported exchange");
                return Ok(());
            }
        };

        //
        // JS callback bridge
        //
        let tsfn = callback.build_threadsafe_function().build()?;

        //
        // clone shared engine
        //
        let inner = self.inner.clone();

        //
        // async subscribe task
        //
        self.runtime.spawn(async move {
            let locked = inner.lock().await;

            locked
                .trades(exchange, &symbol, move |trade| {
                    let payload = serde_json::to_string(&trade).unwrap();

                    tsfn.call(payload, ThreadsafeFunctionCallMode::NonBlocking);
                })
                .await;
        });

        Ok(())
    }
}
