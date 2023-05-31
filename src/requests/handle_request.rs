use serde_json;

use crate::models::request::Request;
use crate::requests::methods::create_cat::create_cat;
use crate::requests::methods::delete_cat::delete_cat;
use crate::requests::methods::get_cat::get_cat;
use crate::requests::methods::get_cats::get_cats;
use crate::requests::methods::update_cat::update_cat;
use crate::responses::generate_responses::{generate_error_response, generate_response};

pub fn handle_request(request: Request) -> String {
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
