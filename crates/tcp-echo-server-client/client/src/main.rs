use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to server at 127.0.0.1:7878");

    let mut stream_clone = stream.try_clone()?;

    // Listen to the server
    let reader_handle = thread::spawn(move || {
        let mut buf = [0u8; 512];
        loop {
            match stream_clone.read(&mut buf) {
                Ok(0) => {
                    println!("Server closed connection");
                    break;
                }
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buf[..n]);
                    println!("Server: {}", msg.trim_end());
                }
                Err(e) => {
                    eprintln!("Failed to read from server: {}", e);
                    break;
                }
            }
        }
    });

    // Read user messages from terminal and send it to the server
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.lock().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }
        stream.write_all(input.as_bytes())?;
    }

    reader_handle.join().unwrap();

    Ok(())
}
