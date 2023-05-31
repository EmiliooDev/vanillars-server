use crate::models::request::{Request, RequestError};
use crate::parses::parse_header::parse_header;
use crate::parses::parse_request_line::parse_request_line;

pub fn parse_request(request: &str) -> Result<Request, RequestError> {
    let mut lines = request.lines();
    let request_line = lines.next().unwrap();
    let (method, url, _version) = parse_request_line(request_line)?;

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
