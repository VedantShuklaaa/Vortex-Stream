mod server;

use server::ws::websocket_router;
use std::sync::Arc;
use tokio::sync::Mutex;
use vortex_stream::{Exchange, VortexStream};

#[tokio::main]
async fn main() {
    //
    // create shared realtime engine
    //
    let stream = Arc::new(Mutex::new(VortexStream::new(vec![
        Exchange::Binance,
        Exchange::Coinbase,
        Exchange::Okx,
    ])));

    //
    // start exchange engines
    //
    {
        let mut locked = stream.lock().await;
        locked.start().await;
    }

    //
    // create axum app
    //
    let app = websocket_router(stream.clone());

    //
    // tcp listener
    //
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("ws server running on 8080");
    axum::serve(listener, app).await.unwrap();
}
