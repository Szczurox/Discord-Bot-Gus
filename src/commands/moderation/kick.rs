use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utils::erorrs::{missing_argument, missing_permission};

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
            // Get user from arguments
            let user = args.single::<UserId>()?;
            // Check if there is no optional argument "reason" 
            if args.is_empty() {
                // Kick member from the guild
                let result = msg.guild_id.unwrap().kick(&ctx.http, user).await;
                // Send message confirming kick if there is no error
                if !result.is_err() {
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully kicked {}", user.mention())).await?;
                } else {
                    missing_permission(msg, ctx, String::from("KICK_MEMBERS_ABOVE")).await;
                }
            } else {
                // Get optional argument "reason"
                let reason = args.single_quoted::<String>().unwrap();
                // Kick member from the guild with a reason
                let result = msg.guild_id.unwrap().kick_with_reason(&ctx.http, user, &reason).await;
                // Send message confirming kick if there is no error
                if !result.is_err() {
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully kicked {} for `{}`", user.mention(), &reason)).await?;
                } else {
                    missing_permission(msg, ctx, String::from("KICK_MEMBERS_ABOVE")).await;
                }
            }
        }
    } else {
        missing_permission(msg, ctx, String::from("KICK_MEMBERS")).await;
    }

    Ok(())
}