use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::infractions::INFRACTION_KICK;
use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::utils::mongo::{add_infraction};
use crate::utils::serenity::{get_discord_tag};
use crate::utils::time::{get_time};

use crate::constants::permissions::PERMISSION_KICK;

// Kick a member out of a guild
// Usage: kick [@member / ID] (reason)
#[command]
#[only_in(guilds)]
pub async fn kick(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
    // Get guild member for author
    let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;

    // Check if author has permission to use this command (KICK_MEMBERS)
    if member.permissions(ctx).expect("permissions").contains(Permissions::KICK_MEMBERS) {
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
                // Kick a member from the guild
                let result = msg.guild_id.unwrap().kick(&ctx.http, user).await;
                // Send a message confirming thekick if there is no error
                if !result.is_err() {
                    // Add the kick to the infractions log
                    let time_stamp: u32 = get_time();
                    let issued_by: String = get_discord_tag(&msg.author);
                    add_infraction(&user.to_string(), &String::from(INFRACTION_KICK), &String::from("reason not provided"), &issued_by, &String::from("Never"), &time_stamp).await;
                    // Send a message confirming the kick
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully kicked {}", user.mention())).await?;
                } else {
                    missing_permission(msg, ctx, String::from(PERMISSION_KICK)).await;
                }
            } else {
                // Get optional argument "reason"
                let reason: String = String::from(args.rest());
                // Kick a member from the guild with a reason
                let result = msg.guild_id.unwrap().kick_with_reason(&ctx.http, user, &reason).await;
                // Send a message confirming the kick if there is no error
                if !result.is_err() {
                    // Add the kick to the infractions log
                    let time_stamp: u32 = get_time();
                    let issued_by: String = get_discord_tag(&msg.author);
                    add_infraction(&user.to_string(), &String::from(INFRACTION_KICK), &reason, &issued_by, &String::from("Never"), &time_stamp).await;
                    // Send a message confirming the kick
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully kicked {} for `{}`", user.mention(), &reason)).await?;
                } else {
                    missing_permission(msg, ctx, String::from(PERMISSION_KICK)).await;
                }
            }
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_KICK)).await;
    }

    Ok(())
}