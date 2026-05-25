use vortex_stream::{Exchange, VortexStream};

#[tokio::main]
async fn main() {
    let mut stream = VortexStream::new(vec![Exchange::Kraken]);
    stream.start().await;

    let _handle = stream
        .trades(Exchange::Kraken, "SOLUSDT", |trade| {
            println!("{:?}", trade);
        })
        .await;

    loop {}
}
