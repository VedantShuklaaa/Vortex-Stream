use vortex_stream::{VortexStream, Exchange};

#[tokio::main]
async fn main() {

    let stream = VortexStream::new(vec![
        Exchange::Coinbase,
    ]);

    stream.trades(
        "SOLUSD",
        |trade| {
            println!("{:?}", trade);
        }
    );

    loop {}
}