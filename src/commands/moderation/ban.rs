use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utils::erorrs::{missing_argument, missing_permission};

// Ban a member from a guild
// Usage: ban [@member / ID] (reason)
#[command]
#[only_in(guilds)]
pub async fn ban(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
    // Get guild member for author
    let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;
    // Check if author has permission to use this command (BAN_MEMBERS)
    if member.permissions(ctx).expect("permissions").contains(Permissions::KICK_MEMBERS) {
        if args.is_empty() {
            missing_argument(msg, ctx, String::from("MEMBER")).await;
        } else {
            // Get user from arguments
            let user = args.single::<UserId>()?;
            // Check if there is no optional argument "reason" 
            if args.is_empty() {
                // Ban member from the guild
                let result = msg.guild_id.unwrap().ban(&ctx.http, user, 0).await;
                // Send message confirming ban if there is no error
                if !result.is_err() {
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully banned {}", user.mention())).await?;
                } else {
                    missing_permission(msg, ctx, String::from("BAN_MEMBERS_ABOVE")).await;
                }
            } else {
                // Get optional argument "reason"
                let reason = args.single_quoted::<String>().unwrap();
                // Ban member from the guild with a reason
                let result = msg.guild_id.unwrap().ban_with_reason(&ctx.http, user, 0, &reason).await;
                // Send message confirming ban if there is no error
                if !result.is_err() {
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully banned {}\nReason: {}", user.mention(), &reason)).await?;
                } else {
                    missing_permission(msg, ctx, String::from("BAN_MEMBERS_ABOVE")).await;
                }
            }
        }
    } else {
        missing_permission(msg, ctx, String::from("KICK MEMBERS")).await;
    }

    Ok(())
}