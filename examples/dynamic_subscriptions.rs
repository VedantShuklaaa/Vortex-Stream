use tokio::time::{Duration, sleep};

use vortex_stream::{Exchange, VortexStream};

#[tokio::main]
async fn main() {
    //
    // initialize stream engine
    //
    let stream = VortexStream::new(vec![Exchange::Binance]);

    //
    // BTC stream
    //
    stream
        .trades(Exchange::Binance, "BTCUSDT", |trade| {
            println!("[BTC] {:?}", trade);
        })
        .await;

    //
    // wait before adding new stream
    //
    sleep(Duration::from_secs(5)).await;

    println!("subscribing to ETHUSDT...");

    //
    // dynamically subscribe ETH
    //
    stream
        .trades(Exchange::Binance, "ETHUSDT", |trade| {
            println!("[ETH] {:?}", trade);
        })
        .await;

    //
    // keep runtime alive
    //
    tokio::signal::ctrl_c().await.unwrap();
}
