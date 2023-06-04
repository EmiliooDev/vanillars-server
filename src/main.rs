use std::io::prelude::*;
use std::net::TcpListener;

mod database;
mod models;
mod parses;
mod private;
mod requests;
mod responses;

use crate::database::connection_to_mongo::connection_to_mongo;
use crate::models::request::RequestError;
use crate::parses::parse_request::parse_request;
use crate::requests::handle_request::handle_request;
use crate::responses::generate_responses::generate_error_response;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on port 8080");

    connection_to_mongo();

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

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received request: \n{}", request);

        let response = match parse_request(&request) {
            Ok(request) => handle_request(request),
            Err(e) => generate_error_response(400, &e.to_string()),
        };

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Error to send the response: {}", e);
        }
    }

    Ok(())
}

impl std::error::Error for RequestError {}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
