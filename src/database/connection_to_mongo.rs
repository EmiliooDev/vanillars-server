use mongodb::{bson::doc, error::Error, sync::Client};
use serde::{Deserialize, Serialize};

use crate::private::MONGO_URL;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

pub fn connection_to_mongo() -> Result<(), Error> {
    let client = Client::with_uri_str(MONGO_URL)?;
    let database = client.database("rapso-cluster");
    let collection = database.collection::<Book>("books");

    let docs = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "Animal Farm".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
        },
    ];

    collection.insert_many(docs, None)?;

    let cursor = collection.find(doc! {"author": "George Orwell"}, None)?;

    for result in cursor {
        println!("title: {}", result?.title);
    }

    Ok(())
}
