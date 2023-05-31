pub fn delete_cat(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    match id {
        "1" | "2" => Ok(()),
        _ => Err("Cat doesn't exists".into()),
    }
}
