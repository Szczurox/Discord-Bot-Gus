use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use crate::utils::{time::get_time, mongo::remove_infraction};

// Infraction log element
#[derive(Debug, Deserialize)]
pub struct Infraction<'a> {
    pub _id: ObjectId,
    #[serde(borrow)]
    pub offender: &'a str,
    #[serde(borrow)]
    pub infraction_type: &'a str,
    #[serde(borrow)]
    pub reason: &'a str,
    #[serde(borrow)]
    pub issued_by: &'a str,
    #[serde(borrow)]
    pub expiration_date: &'a str,
    pub creation_date: u32,
}

impl Infraction<'_> {
    pub async fn check_expiration(&mut self) -> bool {
        // If expiration date already passed
        if self.expiration_date != "Never" && self.expiration_date.parse::<u32>().expect("Parsing error in check_expiration") < get_time() {
            remove_infraction(self._id).await;
            return true;
        }
        return false;
    }
}

pub static INFRACTION_BAN: &str = "BAN";
pub static INFRACTION_KICK: &str = "KICK";
pub static INFRACTION_WARN: &str = "WARN";