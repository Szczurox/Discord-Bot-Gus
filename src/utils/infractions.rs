use std::{sync::Arc, time::Duration};

use mongodb::{bson::{doc, oid::ObjectId, Document}, Cursor};
use serenity::http::Http;
use ticker::Ticker;

use crate::{constants::{infractions::{Infraction, InfractionField, INFRACTION_MUTE}, config::MUTE_ROLE}, utils::time::get_time};

use super::{serenity::remove_role, mongo::get_mongo_db};


// Adds infraction to the database
pub async fn add_infraction(user_id: &String, infraction_type: &String, reason: &String, issued_by: &String, expiration_date: &Option<u32>, creation_date: &u32) {
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

//// Gets one infraction from the database
// pub async fn get_infraction(filter: Document) -> Option<Infraction> {
//     // Get a handle to the database
//    let db = get_mongo_db().unwrap();
//
//    let collection = db.collection::<Infraction>("infractions");
//
//     collection.find_one(
//         filter,
//         None,
//     ).await.expect("Error trying to delete element from the database")
// }

pub async fn infraction_expiration_coroutine(http: &Arc<Http>) {
    let ticker = Ticker::new(0.., Duration::from_secs(1));
    for _ in ticker {
        let mut cursor = get_infractions(doc! { 
            InfractionField::ExpirationDate.as_str(): { 
                "$ne": null,
                "$exists": true,
                "$lte": get_time(),
            }, 
        }).await;

        while cursor.advance().await.expect("Error: Failed to advance cursor in expiration coroutine") {
            let result = cursor.deserialize_current();
            if !result.is_err() {
                let infraction: Infraction = result.unwrap();
                if infraction.infraction_type == INFRACTION_MUTE {
                    remove_infraction(infraction._id).await;
                    remove_role(&http, infraction.offender.parse::<u64>().unwrap(), MUTE_ROLE).await.expect("Error removing mute from member in expiration cooroutine");
                }
            }
        }
    }
}
