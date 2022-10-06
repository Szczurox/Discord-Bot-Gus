use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use mongodb::bson::oid::ObjectId;

use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::constants::permissions::PERMISSION_WARN;
use crate::utils::infractions::remove_infraction_by_id;

// Remove an infraction from the infraction log
// Usage: removewarn [infraction ID]
#[command]
#[only_in(guilds)]
pub async fn removewarn(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
    // Get guild member for author
    let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;
    // Check if author has permission to use this command (MODERATE_MEMBERS)
    if member.permissions(ctx).expect("permissions").contains(Permissions::MODERATE_MEMBERS) {
        if args.is_empty() {
            missing_argument(msg, ctx, String::from("WARN_ID")).await;
        } 
        else {
            let id_result = args.single::<ObjectId>();
            if id_result.is_err() {
                wrong_argument(msg, ctx, String::from("WARN_ID")).await;
                return Ok(());
            }
            let id = id_result.unwrap();
            remove_infraction_by_id(id).await;
            msg.channel_id.say(&ctx.http, &format!("✅ Successfully removed infraction `{}`", id)).await?;
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_WARN)).await;
    }

    Ok(())
}