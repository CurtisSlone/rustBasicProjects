use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use std::env;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
pub async fn main(){
    let url = "ws://127.0.0.1:8080";

    let(ws_stream, _) = connect_async(url).await.expect("Failed");

    println!("Connected to {}", url);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let stdin = io::stdin();
    let handle = tokio::spawn(async move {
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await.expect("Failed") {
            if ws_sender.send(Message::Text(line)).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Text(text) = message {
            println!("{}", text);
        }
    }

    handle.await.expect("Error sending messages");
}