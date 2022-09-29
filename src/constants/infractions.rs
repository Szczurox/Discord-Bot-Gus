use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

// Infraction log element
#[derive(Debug, Deserialize, Clone)]
pub struct Infraction {
    pub _id: ObjectId,
    pub offender: String,
    pub infraction_type: String,
    pub reason: String,
    pub issued_by: String,
    pub expiration_date: Option<u32>,
    pub creation_date: u32,
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