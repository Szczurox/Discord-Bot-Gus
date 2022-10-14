use mongodb::bson::doc;
use serenity::framework::standard::Args;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::economy::{LabField, Lab};
use crate::constants::time::DURATION_TIME;
use crate::utils::economy::get_lab;
use crate::utils::errors::{missing_argument, wrong_argument, lab_doesnt_exist};

pub struct LabValueChangeArgs {
    pub lab: Lab,
    pub user: UserId,
    pub amount: i64,
    pub item_type: String,
}

// Get a duration from some command arguments
pub fn get_duration_from_args(args: &mut Args) -> Option<u32> {
    if !args.is_empty() {
        // Get mute duration time from arguments
        let duration_string: String = args.single::<String>().expect("Error getting duration string");
        // Get time unit (days, months, years, etc.)
        let time_unit: String = duration_string.chars().filter(|c| !c.is_digit(10)).collect();

        // Check if the duration is specified
        if DURATION_TIME.contains_key(&time_unit[..]) {
            // Get number of time units from the mute duration time string
            let duration_length_string: String = duration_string.chars().filter(|c| c.is_digit(10)).collect();
            let duration_length : u32 = duration_length_string.parse::<u32>().unwrap();

            return Some(DURATION_TIME.get(&time_unit[..]).unwrap() * duration_length);
        }
        else {
            args.rewind();
        }
    }

    None
}

// Get a reason from some command arguments
pub fn get_reason_from_args(args: &mut Args) -> String { 
    if args.is_empty() {
        return String::from("reason not provided");
    }
    String::from(args.rest())
}

// Parse value change commands values and return them (or None if there is an error)
pub async fn get_lab_value_change_args(ctx: &Context, msg: &Message, mut args: Args) -> Option<LabValueChangeArgs> {
    // Get the guild member for the author
    let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await.expect("Error getting member for the author from the guild");

    if member.permissions(ctx).expect("permissions").contains(Permissions::ADMINISTRATOR) {
        if args.is_empty() {
            missing_argument(msg, ctx, String::from("TYPE [meth/money]")).await;
        } else {
            let added_item_result = args.single::<String>();
            if added_item_result.is_err() {
                wrong_argument(msg, ctx, String::from("TYPE [meth/money]")).await;
                return None
            }
            let added_item: String = added_item_result.unwrap();
 
            if args.is_empty() {
                missing_argument(msg, ctx, String::from("MEMBER")).await;
                return None
            }

            // Get user from arguments and throw an error if argument is missing
            let user_result = args.single::<UserId>();
            if user_result.is_err() {
                wrong_argument(msg, ctx, String::from("MEMBER")).await;
                return None
            }
            let user: UserId = user_result.unwrap();

            if args.is_empty() {
                missing_argument(msg, ctx, String::from("AMOUNT")).await;
                return None
            }

            let amount_arg = args.single::<i64>();
            if amount_arg.is_err() {
                wrong_argument(msg, ctx, String::from("AMOUNT")).await;
                return None
            }
            let amount = amount_arg.unwrap();

            let lab_arg = get_lab(doc! { 
                LabField::UserId.as_str(): &user.to_string(),
            }).await;

            if lab_arg.is_none() {
                lab_doesnt_exist(&msg, &ctx).await;
                return None
            }
            let lab = lab_arg.unwrap();

            return Some(LabValueChangeArgs { lab: lab, user: user, amount: amount, item_type: added_item })
        }
    }
    None
}