use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use crate::utils::{time::get_time, mongo::remove_infraction};

// Infraction log element
#[derive(Debug, Deserialize, Clone)]
pub struct Infraction {
    pub _id: ObjectId,
    pub offender: String,
    pub infraction_type: String,
    pub reason: String,
    pub issued_by: String,
    pub expiration_date: String,
    pub creation_date: u32,
}

impl Infraction {
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
pub static INFRACTION_MUTE: &str = "MUTE";
pub static INFRACTION_WARN: &str = "WARN";

pub enum InfractionField {
    ID,
    Offender,
    InfractionType,
    Reason,
    IssuedBy,
    ExpirationDate,
    CreationDate,
}

impl InfractionField {
    pub fn as_str(&self) -> &'static str {
        match self {
            InfractionField::ID => "_id",
            InfractionField::Offender => "offender",
            InfractionField::InfractionType => "infraction_type",
            InfractionField::Reason => "reason",
            InfractionField::IssuedBy => "issued_by",
            InfractionField::ExpirationDate => "expiration_date",
            InfractionField::CreationDate => "creation_date",
        }
    }
}