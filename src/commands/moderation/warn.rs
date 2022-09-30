use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::config::DEFAULT_WARN_EXPIRATION;
use crate::constants::infractions::INFRACTION_WARN;
use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::utils::infractions::{add_infraction};
use crate::utils::serenity::{get_discord_tag};
use crate::utils::time::get_time;
use crate::constants::time::DURATION_TIME;
use crate::constants::permissions::PERMISSION_WARN;

// Warn a member
// Usage: warn [@member / ID] (expire in <(num)s, (num)m, (num)h, (num)d, (num)mo, (num)y>, default: 1 month) [reason] 
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
            // Get user from arguments and throw an error if the argument is missing
            let user_result = args.single::<UserId>();
            if user_result.is_err() {
                wrong_argument(msg, ctx, String::from("MEMBER")).await;
                return Ok(())
            }
            let user: UserId = user_result.unwrap();

            if args.is_empty() {
                missing_argument(msg, ctx, String::from("REASON")).await;
            }
            else {
                // Get warn duration time from arguments
                let duration_string: String = args.single::<String>()?;
                // Get time unit (days, months, years, etc.)
                let time_unit: String = duration_string.chars().filter(|c| !c.is_digit(10)).collect();
                let duration: u32;
                // Check if the duration is specified
                if DURATION_TIME.contains_key(&time_unit[..]) {
                    // Get number of time units from the warn duration time string
                    let duration_length_string: String = duration_string.chars().filter(|c| c.is_digit(10)).collect();
                    let duration_length : u32 = duration_length_string.parse::<u32>().unwrap();
                    duration = DURATION_TIME.get(&time_unit[..]).unwrap() * duration_length;
                }
                else {
                    duration = DEFAULT_WARN_EXPIRATION;
                    args.rewind();
                }
                
                let reason: String;
                if args.is_empty() {
                    reason =  String::from("reason not provided");
                } 
                else {
                    reason = String::from(args.rest());
                }

                let time_stamp: u32 = get_time();
                let issued_by: String = get_discord_tag(&msg.author);
                // Warn expiration date (current time + warn duration)
                let expiration: Option<u32> = Some(time_stamp + duration);

                // Add the warn to the infraction log
                add_infraction(&user, &String::from(INFRACTION_WARN), &reason, &issued_by, &expiration, &time_stamp).await;

                // Send a message confirming the warn
                msg.channel_id.say(&ctx.http, &format!("✅ Successfully warned {} for `{}`, expiring on <t:{}:f>", user.mention(), &reason, &expiration.unwrap())).await?;
            }
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_WARN)).await;
    }

    Ok(())
}