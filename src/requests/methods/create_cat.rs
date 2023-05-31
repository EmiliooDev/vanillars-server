use crate::models::request::{Cat, Request};

pub fn create_cat(request: &Request) -> Result<Cat, Box<dyn std::error::Error>> {
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
