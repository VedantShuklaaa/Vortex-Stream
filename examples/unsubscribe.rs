use tokio::time::{Duration, sleep};

use vortex_stream::{Exchange, VortexStream};

#[tokio::main]
async fn main() {
    //
    // initialize stream engine
    //
    let stream = VortexStream::new(vec![Exchange::Binance]);

    //
    // subscribe to BTC
    //
    stream
        .trades(Exchange::Binance, "BTCUSDT", |trade| {
            println!("[BTC] {:?}", trade);
        })
        .await;

    println!("stream active for 10 seconds...");

    //
    // wait before unsubscribe
    //
    sleep(Duration::from_secs(10)).await;

    //
    // unsubscribe
    //
    stream.unsubscribe(Exchange::Binance, "BTCUSDT").await;

    println!("unsubscribed from BTCUSDT");

    //
    // keep process alive
    //
    tokio::signal::ctrl_c().await.unwrap();
}
