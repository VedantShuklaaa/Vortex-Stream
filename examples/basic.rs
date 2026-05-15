use vortex_stream::VortexStream;

#[tokio::main]
async fn main() {

    let stream = VortexStream::new();

    stream.trades(
        "BTCUSDT",
        |trade| {
            println!("{:?}", trade);
        }
    );

    loop {}
}