use crate::models::request::Cat;

pub fn get_cats() -> Result<Vec<Cat>, Box<dyn std::error::Error>> {
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
