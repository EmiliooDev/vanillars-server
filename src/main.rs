use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        let response = "HTTP/1.1 200 OK\r\n\r\nHello World!";
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}
