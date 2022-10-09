use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

// Infraction log element
#[derive(Debug, Deserialize, Clone)]
pub struct Lab {
    pub _id: ObjectId,
    pub user_id: String,
    pub meth: i64,
    pub money: i64,
    pub daily_min: i64,
    pub daily_max: i64,
    pub cooking: Option<u32>,
}

pub enum LabField {
    ID,
    UserId,
    Meth,
    Money,
    DailyMin,
    DailyMax,
    Cooking,
}

impl LabField {
    pub fn as_str(&self) -> &'static str {
        match self {
            LabField::ID => "_id",
            LabField::UserId => "user_id",
            LabField::Meth => "meth",
            LabField::Money => "money",
            LabField::DailyMin => "daily_min",
            LabField::DailyMax => "daily_max",
            LabField::Cooking => "cooking",
        }
    }
}