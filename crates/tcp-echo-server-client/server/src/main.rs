use std::{
    io::{self, BufRead, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
use std::io::Read;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on 127.0.0.1:7878");

    let clients = Arc::new(Mutex::new(Vec::new()));

    // Thread to accept clients
    {
        let clients = Arc::clone(&clients);
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New client connected: {}", stream.peer_addr().unwrap());

                        // Add client to the shared list
                        let mut locked = clients.lock().unwrap();
                        locked.push(stream.try_clone().unwrap());
                        drop(locked);

                        // Clone clients Arc for client thread
                        let clients_for_thread = Arc::clone(&clients);
                        thread::spawn(move || {
                            handle_client(stream, clients_for_thread);
                        });
                    }
                    Err(e) => eprintln!("Failed to accept client: {}", e),
                }
            }
        });
    }

    // Read user messages from terminal and broadcast them to all clients
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.lock().read_line(&mut input)?;
        let msg = input.trim();
        if msg.eq_ignore_ascii_case("exit") {
            println!("Shutting down server.");
            break;
        }

        let mut locked = clients.lock().unwrap();
        // Broadcast message to all clients
        for client in locked.iter_mut() {
            if let Err(e) = client.write_all(format!("Server: {}\n", msg).as_bytes()) {
                eprintln!("Failed to send to client: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let peer = stream.peer_addr().unwrap();
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", peer);
                // Remove client from a list
                let mut locked = clients.lock().unwrap();
                locked.retain(|s| s.peer_addr().ok() != Some(peer));
                break;
            }
            Ok(n) => {
                let msg = &buffer[..n];
                println!("Received from {}: {}", peer, String::from_utf8_lossy(msg));

                // Broadcast client message to all other clients
                let mut locked = clients.lock().unwrap();
                for client in locked.iter_mut() {
                    if client.peer_addr().ok() != Some(peer) {
                        let _ = client.write_all(msg);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", peer, e);
                break;
            }
        }
    }
}
