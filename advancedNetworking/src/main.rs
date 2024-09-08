use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the TCP listener
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    // Create a hashmap to keep track of connected clients
    let mut clients = HashMap::new();

    loop {
        // Accept a new client connection
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        // Spawn a new task to handle the client
        let mut clients_clone = clients.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, &mut clients_clone).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });

        // Update the clients map
        clients.insert(addr, "Connected".to_string());
    }
}

async fn handle_client(mut socket: TcpStream, clients: &mut HashMap<std::net::SocketAddr, String>) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        // Read data from the client
        let n = socket.read(&mut buffer).await?;

        if n == 0 {
            // Connection closed
            break;
        }

        // Handle the received message
        let message = String::from_utf8_lossy(&buffer[..n]);
        println!("Received message: {}", message);

        // Process message
        let response = process_message(&message);

        // Send a response back to the client
        socket.write_all(response.as_bytes()).await?;
    }

    // Remove client from the map
    let addr = socket.peer_addr()?;
    clients.remove(&addr);
    println!("Connection closed from {}", addr);

    Ok(())
}

fn process_message(message: &str) -> String {
    // Simple message processing
    if message.trim() == "PING" {
        "PONG".to_string()
    } else {
        "Unknown command".to_string()
    }
}
