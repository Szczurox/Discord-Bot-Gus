use mongodb::bson::{oid::ObjectId, doc};
use serde::Deserialize;
use serenity::async_trait;

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

#[async_trait]
pub trait Money {
    async fn add_money(&self, amount: &i64);
    async fn remove_money(&self, amount: &i64);
    async fn set_money(&self, amount: &i64);
}

#[async_trait]
pub trait Meth {
    async fn add_meth(&self, amount: &i64);
    async fn remove_meth(&self, amount: &i64);
    async fn set_meth(&self, amount: &i64);
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