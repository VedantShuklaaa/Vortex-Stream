use vortex_stream::{VortexStream, Exchange};

#[tokio::main]
async fn main() {

    let stream = VortexStream::new(vec![
        Exchange::Binance
    ]);

    stream.trades(
        Exchange::Binance,
        "BTCUSDT",
        |trade| {
            println!("{:?}", trade);
        }
    ).await;

    loop {}
}