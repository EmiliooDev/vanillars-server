use crate::models::request::RequestError;

pub fn parse_header(header_line: &str) -> Result<(String, String), RequestError> {
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
