use vortex_stream::{Exchange, VortexStream};

#[tokio::main]
async fn main() {
    let mut stream = VortexStream::new(vec![Exchange::Bitstamp]);
    stream.start().await;

    let _handle = stream
        .trades(Exchange::Bitstamp, "SOLUSDT", |trade| {
            println!("{:?}", trade);
        })
        .await;

    loop {}
}
