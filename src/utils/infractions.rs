use std::{sync::Arc, time::Duration};

use mongodb::{bson::{doc, oid::ObjectId, Document}, Cursor, results::UpdateResult};
use serenity::{http::Http, model::prelude::{UserId, GuildId}};
use ticker::Ticker;

use crate::{constants::{infractions::{Infraction, InfractionField, INFRACTION_MUTE, INFRACTION_BAN, INFRACTION_WARN}, config::{MUTE_ROLE, GUILD_ID}}, utils::time::get_time};

use super::{serenity::remove_role, mongo::get_mongo_db};

// Create infraction document and return it
pub fn infraction_doc(user_id: &UserId, infraction_type: &String, reason: &String, issued_by: &String, expiration_date: &Option<u32>, creation_date: &u32) -> Document {
    doc! {
        InfractionField::Offender.as_str(): user_id.to_string(),
        InfractionField::InfractionType.as_str(): infraction_type,
        InfractionField::Reason.as_str(): reason, 
        InfractionField::IssuedBy.as_str(): issued_by, 
        InfractionField::ExpirationDate.as_str(): expiration_date,
        InfractionField::CreationDate.as_str(): creation_date,
    }
}

// Adds infraction to the database
pub async fn add_infraction(user_id: &UserId, infraction_type: &String, reason: &String, issued_by: &String, expiration_date: &Option<u32>, creation_date: &u32) {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();
    // Add infraction to the database
    db.collection("infractions")
        .insert_one(infraction_doc(user_id, infraction_type, reason, issued_by, expiration_date, creation_date), None)
        .await.expect("Error adding the ban to the infractions log");
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

// Update infraction
pub async fn update_set_infraction(filter: Document, updated: Document) -> UpdateResult {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();

    let collection = db.collection::<Infraction>("infractions");

    collection.update_one(
        filter,
        doc!{"$set": updated},
        None,
    ).await.expect("Error trying to delete element from the database")
}


// // Gets one infraction from the database
// pub async fn get_infraction(filter: Document) -> Option<Infraction> {
//     // Get a handle to the database
//    let db = get_mongo_db().unwrap();

//    let collection = db.collection::<Infraction>("infractions");

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
                    remove_role(&http, UserId::from(infraction.offender.parse::<u64>().unwrap()), MUTE_ROLE).await
                        .expect("Error removing a mute from a member after it expired");
                }
                if infraction.infraction_type == INFRACTION_BAN {
                    remove_infraction(infraction._id).await;
                    GuildId::from(GUILD_ID).unban(&http, UserId::from(infraction.offender.parse::<u64>().unwrap())).await
                        .expect("Error unbanning a member after the ban expired");
                }
                if infraction.infraction_type == INFRACTION_WARN {
                    remove_infraction(infraction._id).await;
                }
            }
        }
    }
}
