use vortex_stream::{Exchange, VortexStream};

#[tokio::main]
async fn main() {
    //
    // initialize multiple exchanges
    //
    let stream = VortexStream::new(vec![Exchange::Binance, Exchange::Coinbase]);

    //
    // Binance stream
    //
    stream
        .trades(Exchange::Binance, "BTCUSDT", |trade| {
            println!("[BINANCE] {:?}", trade);
        })
        .await;

    //
    // Coinbase stream
    //
    stream
        .trades(Exchange::Coinbase, "BTC-USD", |trade| {
            println!("[COINBASE] {:?}", trade);
        })
        .await;

    //
    // keep runtime alive
    //
    tokio::signal::ctrl_c().await.unwrap();

    println!("shutting down...");
}
