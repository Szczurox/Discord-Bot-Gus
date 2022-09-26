use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::constants::permissions::PERMISSION_BAN;

// Unban a member from a guild
// Usage: unban [@member / ID]
#[command]
#[only_in(guilds)]
pub async fn unban(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
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

            // Unban member from the guild
            let result = msg.guild_id.unwrap().unban(&ctx.http, user).await;
            // Send message confirming the unban if there is no error
            if !result.is_err() {
                msg.channel_id.say(&ctx.http, &format!("✅ Successfully unbanned {}", user.mention())).await?;
            } else {
                missing_permission(msg, ctx, String::from(PERMISSION_BAN)).await;
            }
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_BAN)).await;
    }

    Ok(())
}