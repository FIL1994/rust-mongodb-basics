#[macro_use(bson, doc)]
extern crate bson;
extern crate dotenv;
extern crate mongodb;

use bson::Bson;
use dotenv::dotenv;
use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};
use std::env;

fn main() {
    dotenv().ok();

    let uri = env::var("MONGO_URL").expect("MONGO_URL must be set");

    let client = Client::with_uri(&uri)
        .expect("Failed to initialize standalone client.");

    let coll = client.db("mydb").collection("movies");

    let doc = doc! {
        "title": "Other",
    };

    // coll.insert_one(doc.clone(), None)
    //     .ok()
    //     .expect("Failed to insert document");

    let mut cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find.");

    let item = cursor.next();

    match item {
        Some(Ok(doc)) => match doc.get("title") {
            Some(&Bson::String(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }

    cursor = coll
        .find(Some(doc!{}), None)
        .ok()
        .expect("Failed to execute find");

    for item in cursor {
        match item {
            Ok(doc) => match doc.get("title") {
                Some(&Bson::String(ref title)) => println!("{}", title),
                _ => panic!("Expected title to be a string!"),
            },
            Err(_) => panic!("Server returned no results"),
        }
    }
}
