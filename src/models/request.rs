use serde::{Deserialize, Serialize};

pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cat {
    pub id: String,
    pub name: String,
    pub breed: String,
    pub age: i32,
    pub owner: String,
    pub address: String,
}

#[derive(Debug)]
pub struct RequestError {
    pub message: String,
}
