use mongodb::{options::ClientOptions, bson::{doc, oid::ObjectId}};

use crate::constants::infractions::Infraction;

static mut MONGO_CLIENT: Option<mongodb::Client> = None;

// Returns database handle
pub fn get_mongo_db() -> Option<mongodb::Database> {
    unsafe {
       Some(MONGO_CLIENT.to_owned().unwrap().database("main-db"))
    }
}

// Initializes mongo client
pub async fn init_mongo_client(connection_string: String) {
    // Parse onnection string into an options struct
    let mut client_options =
    ClientOptions::parse(connection_string)
        .await.expect("Error parsing connection string");
    client_options.app_name = Some("GusBot".to_string());
    unsafe {
        MONGO_CLIENT = Some(mongodb::Client::with_options(client_options).expect("Error getting cluster handle"));
    }
}

// Adds infraction to the database
pub async fn add_infraction(user_id: &String, infraction_type: &String, reason: &String, issued_by: &String, expiration_date: &String, creation_date: &u32) {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();

    // Add infraction to the database
    db.collection("infractions").insert_one(doc! {
        "offender": &user_id,
        "infraction_type": &infraction_type,
        "reason": &reason, 
        "issued_by": &issued_by, 
        "expiration_date": &expiration_date,
        "creation_date": &creation_date,
    }, None).await.expect("Error adding the ban to the infractions log");
}

// Removes infraction from the database
pub async fn remove_infraction(id: ObjectId) {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();

    let collection = db.collection::<Infraction>("infractions");
    // Remove infraction to the database
    collection.delete_one(
        doc! {
            "_id": id
        }, 
        None,
    ).await.expect("Error trying to delete element from the database");
}