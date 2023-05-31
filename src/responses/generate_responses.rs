use crate::responses::status_message::status_message;

pub fn generate_response(status_code: u16, content_type: &str, body: String) -> String {
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

pub fn generate_error_response(status_code: u16, message: &str) -> String {
    generate_response(status_code, "text/plain", message.to_owned())
}
