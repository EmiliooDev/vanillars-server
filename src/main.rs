use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on port 8080");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Error to accept the connection: {}", e);
                continue;
            }
        };

        let mut buffer = [0; 1024];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                eprintln!("Error to read the data: {}", e);
                continue;
            }
        };

        let response = "HTTP/1.1 200 OK\r\n\r\nHello World!";
        match stream.write_all(response.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error to send the response: {}", e);
            }
        };
    }
    Ok(())
}
