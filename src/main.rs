use serde::{Deserialize, Serialize};
use serde_json;
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

fn parse_request(request: &str) -> Result<Request, RequestError> {
    let mut lines = request.lines();
    let request_line = lines.next().unwrap();
    let (method, url, version) = parse_request_line(request_line)?;

    let mut headers = Vec::new();
    while let Some(header_line) = lines.next() {
        if header_line.trim().is_empty() {
            break;
        }
        let (name, value) = parse_header(header_line)?;
        headers.push((name, value));
    }

    let body = match lines.next() {
        Some("") => {
            let body_lines: Vec<_> = lines.collect();
            Some(body_lines.join("\n"))
        }
        Some(line) => Some(line.to_owned()),
        None => None,
    };

    // last code |^.^|
    // let mut body = None;
    // if let Some("") = lines.next() {
    //     let mut body_lines = Vec::new();
    //     for line in lines {
    //         body_lines.push(line);
    //     }
    //     body = Some(body_lines.join("\n"));
    // }

    Ok(Request {
        method,
        url,
        headers,
        body,
    })
}

fn parse_request_line(request_line: &str) -> Result<(String, String, String), RequestError> {
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or(RequestError {
            message: String::from("Invalid request line"),
        })?
        .to_owned();
    let url = parts
        .next()
        .ok_or(RequestError {
            message: String::from("Invalid request line"),
        })?
        .to_owned();
    let version = parts
        .next()
        .ok_or(RequestError {
            message: String::from("Invalid request line"),
        })?
        .to_owned();
    Ok((method, url, version))
}

fn parse_header(header_line: &str) -> Result<(String, String), RequestError> {
    let mut parts = header_line.splitn(2, ':');
    let name = parts
        .next()
        .ok_or(RequestError {
            message: String::from("Invalid header line"),
        })?
        .trim()
        .to_owned();
    let value = parts
        .next()
        .ok_or(RequestError {
            message: String::from("Invalid header line"),
        })?
        .trim()
        .to_owned();

    Ok((name, value))
}

fn handle_request(request: Request) -> String {
    match (request.method.as_str(), request.url.as_str()) {
        ("GET", "/cat") => {
            let cats_result = get_cats();
            let cats = match cats_result {
                Ok(cats) => cats,
                Err(e) => {
                    let error_msg = e.to_string();
                    return generate_error_response(500, &error_msg);
                }
            };

            let body = serde_json::to_string(&cats)
                .unwrap_or_else(|e| generate_error_response(500, &e.to_string()));

            generate_response(200, "application/json", body)
        }
        ("POST", "/cat") => {
            let cat_result = create_cat(&request);
            let cat = match cat_result {
                Ok(cat) => cat,
                Err(e) => {
                    let error_msg = e.to_string();
                    return generate_error_response(400, &error_msg);
                }
            };

            let body = serde_json::to_string(&cat)
                .unwrap_or_else(|e| generate_error_response(500, &e.to_string()));

            generate_response(201, "application/json", body)
        }
        ("GET", path) if path.starts_with("/cat/") => {
            let id = path.trim_start_matches("/cat/");
            let cat_result = get_cat(id);
            let cat = match cat_result {
                Ok(cat) => cat,
                Err(e) => {
                    let error_msg = e.to_string();
                    return generate_error_response(404, &error_msg);
                }
            };

            let body = serde_json::to_string(&cat)
                .unwrap_or_else(|e| generate_error_response(500, &e.to_string()));

            generate_response(200, "application/json", body)
        }
        ("PUT", path) if path.starts_with("/cat") => {
            let id = path.trim_start_matches("/cat/");
            let cat_result = update_cat(id, &request);
            let cat = match cat_result {
                Ok(cat) => cat,
                Err(e) => {
                    let error_msg = e.to_string();
                    return generate_error_response(400, &error_msg);
                }
            };

            let body = serde_json::to_string(&cat)
                .unwrap_or_else(|e| generate_error_response(500, &e.to_string()));

            generate_response(200, "application/json", body)
        }
        ("DELETE", path) if path.starts_with("/cat/") => {
            let id = path.trim_start_matches("/cat/");
            let delete_result = delete_cat(id);
            if let Err(e) = delete_result {
                let error_msg = e.to_string();
                return generate_error_response(404, &error_msg);
            }

            generate_response(204, "", "".to_owned())
        }
        _ => generate_error_response(404, "Not found"),
    }
}

fn generate_response(status_code: u16, content_type: &str, body: String) -> String {
    let mut response = String::new();
    response.push_str(&format!(
        "HTTP/1.1 {} {}\r\n",
        status_code,
        status_message(status_code)
    ));
    response.push_str(&format!("Content-Type: {}\r\n", content_type));
    response.push_str("\r\n");
    response.push_str(&body);

    response
}

fn get_cats() -> Result<Vec<Cat>, Box<dyn std::error::Error>> {
    //example get cats from a database
    let cats = vec![
        Cat {
            id: "1".to_owned(),
            name: "Lilith".to_owned(),
            breed: "Persian".to_owned(),
            age: 2,
            owner: "Emilio Ortiz".to_owned(),
            address: "Ostra 295".to_owned(),
        },
        Cat {
            id: "2".to_owned(),
            name: "Nitara".to_owned(),
            breed: "Siamese".to_owned(),
            age: 1,
            owner: "Emilia Dominguez".to_owned(),
            address: "Marlin 218".to_owned(),
        },
    ];

    Ok(cats)
}

fn create_cat(request: &Request) -> Result<Cat, Box<dyn std::error::Error>> {
    let cat = Cat {
        id: "3".to_owned(),
        name: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.name)
            .unwrap_or_else(|| "".to_owned()),
        breed: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.breed)
            .unwrap_or_else(|| "".to_owned()),
        age: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.age)
            .unwrap_or(0),
        owner: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.owner)
            .unwrap_or_else(|| "".to_owned()),
        address: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.address)
            .unwrap_or_else(|| "".to_owned()),
    };
    Ok(cat)
}

fn get_cat(id: &str) -> Result<Cat, Box<dyn std::error::Error>> {
    let cat = match id {
        "1" => Cat {
            id: "1".to_owned(),
            name: "Kitty".to_owned(),
            breed: "Persian".to_owned(),
            age: 2,
            owner: "John Doe".to_owned(),
            address: "123 Main St".to_owned(),
        },
        "2" => Cat {
            id: "2".to_owned(),
            name: "Fluffy".to_owned(),
            breed: "Siamese".to_owned(),
            age: 3,
            owner: "Jane Smith".to_owned(),
            address: "456 Park Ave".to_owned(),
        },
        _ => return Err("Gato no encontrado".into()),
    };
    Ok(cat)
}

fn update_cat(id: &str, request: &Request) -> Result<Cat, Box<dyn std::error::Error>> {
    let cat = Cat {
        id: id.to_owned(),
        name: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.name)
            .unwrap_or_else(|| "".to_owned()),
        breed: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.breed)
            .unwrap_or_else(|| "".to_owned()),
        age: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.age)
            .unwrap_or(0),
        owner: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.owner)
            .unwrap_or_else(|| "".to_owned()),
        address: request
            .body
            .as_ref()
            .and_then(|b| serde_json::from_str::<Cat>(b).ok())
            .map(|c| c.address)
            .unwrap_or_else(|| "".to_owned()),
    };
    Ok(cat)
}

fn delete_cat(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    match id {
        "1" | "2" => Ok(()),
        _ => Err("Cat doesn't exists".into()),
    }
}

fn generate_error_response(status_code: u16, message: &str) -> String {
    generate_response(status_code, "text/plain", message.to_owned())
}

fn status_message(status_code: u16) -> &'static str {
    match status_code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Cat {
    id: String,
    name: String,
    breed: String,
    age: i32,
    owner: String,
    address: String,
}

struct Request {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Debug)]
struct RequestError {
    message: String,
}

impl std::error::Error for RequestError {}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
