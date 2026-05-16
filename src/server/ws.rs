use std::sync::Arc;

use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};

use tokio::sync::{Mutex, mpsc};

use crate::{Exchange, VortexStream};

type SharedStream = Arc<Mutex<VortexStream>>;

pub async fn ws_handler(
    State(stream): State<SharedStream>,

    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, stream))
}

async fn handle_socket(mut socket: WebSocket, stream: SharedStream) {
    println!("client connected");

    //
    // outgoing websocket channel
    //
    let (tx, mut rx) = mpsc::channel::<String>(100);

    //
    // websocket writer task
    //
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let _ = socket.send(Message::Text(message.into())).await;
        }
    });

    //
    // lock vortex engine
    //
    let locked = stream.lock().await;

    //
    // subscribe to trades
    //
    locked
        .trades(Exchange::Binance, "BTCUSDT", move |trade| {
            let payload = serde_json::to_string(&trade).unwrap();

            let tx_clone = tx.clone();

            tokio::spawn(async move {
                let _ = tx_clone.send(payload).await;
            });
        })
        .await;
}

pub fn websocket_router(stream: SharedStream) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(stream)
}
