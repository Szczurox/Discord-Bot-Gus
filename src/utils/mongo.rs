use mongodb::{options::ClientOptions};

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