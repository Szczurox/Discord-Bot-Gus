use mongodb::bson::doc;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::config::{MUTE_ROLE};
use crate::constants::infractions::{INFRACTION_MUTE, Infraction, InfractionField};
use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::constants::permissions::PERMISSION_MUTE;
use crate::utils::infractions::{remove_infraction, get_infractions};

// Unmute a member of a guild
// Usage: unmute [@member / ID]
#[command]
#[only_in(guilds)]
pub async fn unmute(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
    // Get the guild member for the author
    let author_member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;

    // Check if author has permission to use this command (KICK_MEMBERS)
    if author_member.permissions(ctx).expect("permissions").contains(Permissions::MODERATE_MEMBERS) {
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
    
            // Remove mute from the infraction log
            let mut mutes = get_infractions(doc! { 
                InfractionField::InfractionType.as_str(): &String::from(INFRACTION_MUTE), 
            }).await;

            while mutes.advance().await? {
                let mute: Infraction = mutes.deserialize_current().unwrap();
                remove_infraction(mute._id).await;
            }

            // Get a guild member for the user
            let mut member = msg.guild_id.unwrap().member(&ctx.http, user).await?;
            member.remove_role(&ctx.http, MUTE_ROLE).await?;

            msg.channel_id.say(&ctx.http, &format!("✅ Successfully unmuted {} for ", user.mention())).await?;
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_MUTE)).await;
    }

    Ok(())
}