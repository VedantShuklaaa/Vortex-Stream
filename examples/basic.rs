use vortex_stream::{VortexStream, Exchange};

#[tokio::main]
async fn main() {

    let stream = VortexStream::new(vec![
        Exchange::Binance
    ]);

    stream.trades(
        "BTCUSDT",
        |trade| {
            println!("{:?}", trade);
        }
    );

    loop {}
}