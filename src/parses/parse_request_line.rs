use crate::models::request::RequestError;

pub fn parse_request_line(request_line: &str) -> Result<(String, String, String), RequestError> {
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
