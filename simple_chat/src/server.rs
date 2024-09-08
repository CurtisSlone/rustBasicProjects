use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;

use futures_util::{SinkExt, StreamExt};

async fn handle_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>,
    mut tx: broadcast::Sender<String>,
) {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            if ws_sender.send(Message::Text(message)).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
        let _ = tx.send(text);
    }
}

#[tokio::main]
pub async fn main(){
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to get listener");
    let (tx,_) = broadcast::channel(100);

    println!("Server running on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

        tokio::spawn(handle_connection(ws_stream, tx));
    }
}