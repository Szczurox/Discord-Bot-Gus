use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::infractions::INFRACTION_BAN;
use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::utils::mongo::{add_infraction};
use crate::utils::serenity::{get_discord_tag};
use crate::utils::time::{get_time};
use crate::constants::permissions::PERMISSION_BAN;

// Ban a member from a guild
// Usage: ban [@member / ID] (reason)
#[command]
#[only_in(guilds)]
pub async fn ban(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
    // Get guild member for author
    let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;
    // Check if author has permission to use this command (BAN_MEMBERS)
    if member.permissions(ctx).expect("permissions").contains(Permissions::BAN_MEMBERS) {
        if args.is_empty() {
            missing_argument(msg, ctx, String::from("MEMBER")).await;
        } else {
            // Get user from arguments and throw an error if argument is missing
            let user_result = args.single::<UserId>();
            if user_result.is_err() {
                wrong_argument(msg, ctx, String::from("MEMBER")).await;
                return Ok(())
            }
            let user: UserId = user_result.unwrap();
                        
            // Check if there is no optional argument "reason" 
            if args.is_empty() {
                // Ban a member from the guild
                let result = msg.guild_id.unwrap().ban(&ctx.http, user, 0).await;
                // Send a message confirming th ban if there is no error
                if !result.is_err() {
                    // Add the ban to the infractions log
                    let time_stamp: u32 = get_time();
                    let issued_by: String = get_discord_tag(&msg.author);
                    add_infraction(&user.to_string(), &String::from(&String::from(INFRACTION_BAN)), 
                                    &String::from("reason not provided"), &issued_by, &String::from("Never"), &time_stamp).await;
                    // Send a message confirming the ban
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully banned {}", user.mention())).await?;
                } else {
                    missing_permission(msg, ctx, String::from(PERMISSION_BAN)).await;
                }
            } else {
                // Get optional argument "reason"
                let reason = String::from(args.rest());
                // Ban a member from the guild with a reason
                let result = msg.guild_id.unwrap().ban_with_reason(&ctx.http, user, 0, &reason).await;
                // Send a message confirming the ban if there is no error and add ban to infractions log
                if !result.is_err() {
                    // Add the ban to the infractions log
                    let time_stamp: u32 = get_time();
                    let issued_by: String = get_discord_tag(&msg.author);
                    add_infraction(&user.to_string(), &String::from(&String::from(INFRACTION_BAN)), 
                                    &reason, &issued_by, &String::from("Never"), &time_stamp).await;
                    // Send a message confirming the ban
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully banned {}\nReason: {}", user.mention(), &reason)).await?;
                } else {
                    missing_permission(msg, ctx, String::from(PERMISSION_BAN)).await;
                }
            }
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_BAN)).await;
    }

    Ok(())
}