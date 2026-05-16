use vortex_stream::{Exchange, VortexStream};

#[tokio::main]
async fn main() {
    //
    // initialize realtime engine
    //
    let stream = VortexStream::new(vec![Exchange::Binance]);

    //
    // subscribe to BTCUSDT trades
    //
    stream
        .trades(Exchange::Binance, "BTCUSDT", |trade| {
            println!("[BTCUSDT] {:?}", trade);
        })
        .await;

    //
    // keep process alive
    //
    tokio::signal::ctrl_c().await.unwrap();

    println!("shutting down...");
}
