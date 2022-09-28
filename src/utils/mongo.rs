use mongodb::{options::{ClientOptions}, bson::{doc, oid::ObjectId, Document}, Cursor};

use crate::constants::infractions::{Infraction, InfractionField};

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
        InfractionField::Offender.as_str(): &user_id,
        InfractionField::InfractionType.as_str(): &infraction_type,
        InfractionField::Reason.as_str(): &reason, 
        InfractionField::IssuedBy.as_str(): &issued_by, 
        InfractionField::ExpirationDate.as_str(): &expiration_date,
        InfractionField::CreationDate.as_str(): &creation_date,
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
            InfractionField::ID.as_str(): id
        }, 
        None,
    ).await.expect("Error trying to delete element from the database");
}

// Gets multiple infractions from the database
pub async fn get_infractions(filter: Document) -> Cursor<Infraction> {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();

    let collection = db.collection::<Infraction>("infractions");

    collection.find(
        filter,
        None,
    ).await.expect("Error trying to delete element from the database")
}

// Gets one infraction from the database
pub async fn get_infraction(filter: Document) -> Option<Infraction> {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();

    let collection = db.collection::<Infraction>("infractions");

    collection.find_one(
        filter,
        None,
    ).await.expect("Error trying to delete element from the database")
}