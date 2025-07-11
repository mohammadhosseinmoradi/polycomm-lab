use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");
    println!("Server running at ws://127.0.0.1:8080");

    let (tx, _) = broadcast::channel::<Vec<u8>>(100);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New client connected: {}", addr);
        handle_connection(stream, addr, tx.clone()).await;
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    tx: broadcast::Sender<Vec<u8>>,
) {
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        let ws_stream = match accept_async(stream).await {
            Ok(ws) => ws,
            Err(_) => {
                println!("Error during websocket handshake for {}", addr);
                return;
            }
        };

        let (mut write, mut read) = ws_stream.split();
        let mut is_broadcaster = false;

        loop {
            tokio::select! {
                msg = read.next() => {
                    match msg {
                        Some(Ok(m)) => {
                            handle_message(&m, &addr, &tx, &mut is_broadcaster);
                        }
                        _ => break,
                    }
                }
                Ok(frame) = rx.recv(), if !is_broadcaster => {
                    if write.send(frame.into()).await.is_err() {
                        break;
                    }
                }
            }
        }

        println!("Client disconnected: {}", addr);
    });
}

fn handle_message(
    message: &tokio_tungstenite::tungstenite::Message,
    addr: &std::net::SocketAddr,
    tx: &broadcast::Sender<Vec<u8>>,
    is_broadcaster: &mut bool,
) {
    if message.is_text() && message.to_text().unwrap() == "BROADCAST" {
        *is_broadcaster = true;
        println!("Client {} became broadcaster", addr);
    } else if message.is_binary() && *is_broadcaster {
        let _ = tx.send(message.clone().into_data().to_vec());
    }
}
