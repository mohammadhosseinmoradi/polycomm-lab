use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to 127.0.0.1:8080");
    println!("Listening on ws://127.0.0.1:8080");

    let (tx, _) = broadcast::channel::<(Vec<u8>, usize)>(100);

    let mut client_id_counter = 0;

    while let Ok((stream, add)) = listener.accept().await {
        println!("New client {}", add);
        
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let client_id = client_id_counter;
        client_id_counter += 1;

        tokio::spawn(async move {
            let websocket_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = websocket_stream.split();

            let write_task = tokio::spawn(async move {
                while let Ok((data, sender_id)) = rx.recv().await {
                    if sender_id == client_id {
                        continue;
                    }
                    if write.send(data.into()).await.is_err() {
                        break;
                    };
                }
            });

            while let Some(Ok(msg)) = read.next().await {
                if let Message::Binary(data) = msg {
                    let _ = tx.send((data.into(), client_id));
                }
            }
            
            write_task.abort();

            println!("Client {} disconnected", add);
        });
    }
}
