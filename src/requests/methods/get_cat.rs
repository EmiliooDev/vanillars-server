use crate::models::request::Cat;

pub fn get_cat(id: &str) -> Result<Cat, Box<dyn std::error::Error>> {
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
        _ => return Err("Cat not found.".into()),
    };
    Ok(cat)
}
