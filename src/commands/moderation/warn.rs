use std::time::SystemTime;
use mongodb::{bson::doc};

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utils::erorrs::{missing_argument, missing_permission};
use crate::utils::mongo::{get_mongo_db};
use crate::constants::DURATION_TIME;

// Warn a member
// Usage: warn [@member / ID] (expire in <(num)s, (num)m, (num)h, (num)d, (num)mo, (num)y, never>, default: 1 month) [reason] 
#[command]
#[only_in(guilds)]
pub async fn warn(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
    // Get guild member for author
    let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;
    // Check if author has permission to use this command (MODERATE_MEMBERS)
    if member.permissions(ctx).expect("permissions").contains(Permissions::MODERATE_MEMBERS) {
        if args.is_empty() {
            missing_argument(msg, ctx, String::from("MEMBER")).await;
        } 
        else {
            // Get user from arguments
            let user: UserId = args.single::<UserId>()?;
            if args.is_empty() {
                missing_argument(msg, ctx, String::from("REASON")).await;
            }
            else {
                // Get warn duration time from arguments
                let duration_string: String = args.single::<String>()?;
                // Get time unit (days, months, years, etc.)
                let time_unit: String = duration_string.chars().filter(|c| !c.is_digit(10)).collect();
                // Set default duration (1 month)
                let mut duration: u32 = 2630000;

                // Check if the duration is specified
                if DURATION_TIME.contains_key(&time_unit[..]) {
                    // Get number of time units from the warn duration time string
                    let duration_length_string: String = duration_string.chars().filter(|c| c.is_digit(10)).collect();
                    let duration_length : u32 = duration_length_string.parse::<u32>().unwrap();
                    // Set the duration to the time unit converted into seconds multiplied by the number of time units
                    duration = DURATION_TIME.get(&time_unit[..]).unwrap() * duration_length;
                }
                else {
                    // Rewing argument if the duration is not specified
                    args.rewind();
                }
                
                // If the reason was treated as duration time (or there was no reason)
                if args.is_empty() {
                    // Set the duration time back to default and change the argument to a reason
                    duration = 2630000;
                    args.rewind();
                }

                // Get warn reason from arguments
                let reason: String = args.single_quoted::<String>().unwrap();

                // Get current time in unix time
                let time_stamp: u32 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(n) => n.as_secs() as u32,
                    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                };
                
                // Create issuing member username + tag string
                let mut issued_by: String = msg.author.name.to_owned();
                issued_by.push_str("#");
                // Discriminator is an unsigned int, so 0s at the beginning (if they exist) have to be added manually
                let discriminator: u16 = msg.author.discriminator;
                if discriminator < 1000 {
                    issued_by.push_str("0");
                }
                if discriminator < 100 {
                    issued_by.push_str("0");
                }
                if discriminator < 10 {
                    issued_by.push_str("0");
                }
                issued_by.push_str(&discriminator.to_string());

                // Warn expiration date (current time + warn duration)
                let expiration: u32 = time_stamp + duration;
                
                // Get a handle to the database
                let db = get_mongo_db().unwrap();

                // Add warn to the database
                db.collection("infractions").insert_one(doc! {
                    "offender": &user.to_string(),
                    "type": "Warn",
                    "reason": &reason, 
                    "issued-by": &issued_by, 
                    "expiring": &expiration, 
                    "creation-date": &time_stamp, 
                }, None).await.expect("Error adding the warning");

                // Send message confirming warn
                msg.channel_id.say(&ctx.http, &format!("✅ Successfully warned {} for `{}`, expiring on <t:{}:f>", user.mention(), &reason, &expiration)).await?;
            }
        }
    } else {
        missing_permission(msg, ctx, String::from("KICK_MEMBERS")).await;
    }

    Ok(())
}