use vortex_stream::{Exchange, VortexStream};

#[tokio::main]
async fn main() {
    let mut stream = VortexStream::new(vec![Exchange::CryptoCom]);
    stream.start().await;

    let _handle = stream
        .trades(Exchange::CryptoCom, "BTCUSDT", |trade| {
            println!("{:?}", trade);
        })
        .await;

    loop {}
}
