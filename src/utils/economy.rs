use mongodb::bson::{doc, Document};
use serenity::{model::prelude::*, async_trait};

use crate::constants::economy::{Lab, LabField, Money, Meth};

use super::mongo::get_mongo_db;

// Create lab document and return it
pub fn lab_doc(user_id: UserId, meth: i64, money: i64, daily_min: i64, daily_max: i64, cooking: Option<u32>) -> Document {
    doc! {
        LabField::UserId.as_str(): user_id.to_string(),
        LabField::Meth.as_str(): meth,
        LabField::Money.as_str(): money,
        LabField::DailyMin.as_str(): daily_min,
        LabField::DailyMax.as_str(): daily_max,
        LabField::Cooking.as_str(): cooking,
    }
}

// Adds lab to the database
pub async fn lab_setup(user_id: UserId) {
    // Get a handle to the database
    let db = get_mongo_db().unwrap();
    lab_doc(user_id, 0, 1000, 1, 3, None::<u32>);
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

#[async_trait]
impl Money for Lab {
    // Adds specified amount of money to the lab
    async fn add_money(&self, amount : &i64) {
        // Get a handle to the database
        let db = get_mongo_db().unwrap();

        let collection = db.collection::<Lab>("economy");

        collection.update_one(
            doc! { 
                LabField::ID.as_str(): &self._id,
            },
            doc!{"$set": doc! { LabField::Money.as_str() : &self.money + amount }},
            None,
        ).await.expect("Error trying to update money");
    }

    // Removes specified amount of money from the lab
    async fn remove_money(&self, amount : &i64) {
        self.add_money(&(-amount)).await;
    }

    // Sets amount of money of the lab to specified
    async fn set_money(&self, amount : &i64) {
        // Get a handle to the database
        let db = get_mongo_db().unwrap();

        let collection = db.collection::<Lab>("economy");

        collection.update_one(
            doc! { 
                LabField::ID.as_str(): &self._id,
            },
            doc!{"$set": doc! { LabField::Money.as_str() : amount }},
            None,
        ).await.expect("Error trying to update meth");
    }
}

#[async_trait]
impl Meth for Lab {
    // Adds specified amount of meth to the lab
    async fn add_meth(&self, amount : &i64) {
        // Get a handle to the database
        let db = get_mongo_db().unwrap();

        let collection = db.collection::<Lab>("economy");

        collection.update_one(
            doc! { 
                LabField::ID.as_str(): &self._id,
            },
            doc!{"$set": doc! { LabField::Meth.as_str() : &self.meth + amount }},
            None,
        ).await.expect("Error trying to update meth");
    }

    // Removes specified amount of meth from the lab
    async fn remove_meth(&self, amount : &i64) {
        self.add_meth(&(-amount)).await;
    }

    // Sets amount of meth of the lab to specified
    async fn set_meth(&self, amount : &i64) {
        // Get a handle to the database
        let db = get_mongo_db().unwrap();

        let collection = db.collection::<Lab>("economy");

        collection.update_one(
            doc! { 
                LabField::ID.as_str(): &self._id,
            },
            doc!{"$set": doc! { LabField::Meth.as_str() : amount }},
            None,
        ).await.expect("Error trying to update meth");
    }
}