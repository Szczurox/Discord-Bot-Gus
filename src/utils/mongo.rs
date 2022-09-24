use mongodb::{options::ClientOptions, bson::doc};

static mut MONGO_CLIENT: Option<mongodb::Client> = None;

pub fn get_mongo_db() -> Option<mongodb::Database> {
    unsafe {
       Some(MONGO_CLIENT.to_owned().unwrap().database("main-db"))
    }
}

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

pub async fn add_infraction(user_id: &String, infraction_type: &String, reason: &String, issued_by: &String, expiration_date: &String, creation_date: &u32) {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();

    // Add warn to the database
    db.collection("infractions").insert_one(doc! {
        "offender": &user_id,
        "type": &infraction_type,
        "reason": &reason, 
        "issued-by": &issued_by, 
        "expiring": &expiration_date,
        "creation-date": &creation_date,
    }, None).await.expect("Error adding the ban to the infractions log");
}