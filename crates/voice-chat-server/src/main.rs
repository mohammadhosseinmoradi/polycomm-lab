use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (tx, _) = broadcast::channel::<Vec<u8>>(100);

    println!("Server listening on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();

            // Spawn task to forward received broadcast messages to this client
            let mut write_task = tokio::spawn(async move {
                while let Ok(data) = rx.recv().await {
                    if write.send(data.into()).await.is_err() {
                        break;
                    }
                }
            });

            // Read incoming messages and broadcast them
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Binary(data) = msg {
                    let _ = tx.send(data.to_vec());
                }
            }

            write_task.abort();
        });
    }
}
