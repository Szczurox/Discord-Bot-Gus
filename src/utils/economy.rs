use mongodb::bson::{doc, Document};
use serenity::model::prelude::*;

use crate::constants::economy::{Lab, LabField};

use super::mongo::get_mongo_db;

// Adds lab to the database
pub async fn lab_setup(user_id: &UserId) {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();

    // Add lab to the database
    db.collection("economy").insert_one(doc! {
            LabField::UserId.as_str(): user_id.to_string(),
            LabField::Meth.as_str(): 0,
            LabField::Money.as_str(): 1000,
            LabField::DailyMin.as_str(): 1,
            LabField::DailyMax.as_str(): 3,
            LabField::Cooking.as_str(): null,
        }, None)
        .await.expect("Error adding a Lab to the economy collection");
}



// Gets a lab from the database
pub async fn get_lab(filter: Document) -> Option<Lab> {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();

    let collection = db.collection::<Lab>("economy");

    collection.find_one(
        filter,
        None,
    ).await.expect("Error trying to get element from the database")
}
