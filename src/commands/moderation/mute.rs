use mongodb::bson::doc;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::config::{MUTE_ROLE};
use crate::constants::infractions::{INFRACTION_MUTE, InfractionField};
use crate::utils::args::{get_reason_from_args, get_duration_from_args};
use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::constants::permissions::PERMISSION_MUTE;
use crate::utils::infractions::{add_infraction, update_set_infraction, infraction_doc};
use crate::utils::serenity::{get_discord_tag, add_role, check_role};
use crate::utils::time::get_time;

// Mute a member of a guild
// Usage: mute [@member / ID] (duration <(num)s, (num)m, (num)h, (num)d, (num)mo, (num)y>, if not specified then mute will last forever) (reason)
#[command]
#[only_in(guilds)]
pub async fn mute(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
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

            let duration: Option<u32> = get_duration_from_args(&mut args);

            let reason: String = get_reason_from_args(&mut args);
            
            let time_stamp: u32 = get_time();

            let issued_by: String = get_discord_tag(&msg.author);
            
            let expiration: Option<u32>;
            if duration != None {
                // Mute end (current time + warn duration)
                expiration = Some(time_stamp + &duration.unwrap());
            } else {
                expiration = None;
            }

            // Check if member is already muted
            if !check_role(&ctx.http, user, MUTE_ROLE).await {
                // Set mute and add it to the infraction log
                add_infraction(&user, &String::from(INFRACTION_MUTE), &reason, &issued_by, &expiration, &time_stamp).await;
                add_role(&ctx.http, user, MUTE_ROLE).await?;

                if duration != None {
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully muted {} for `{}`, expiring on <t:{}:f>", user.mention(), &reason, &expiration.unwrap())).await?;
                }
                else {
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully muted {} for `{}`, lasting `Forever`", user.mention(), &reason)).await?;
                }
            }
            else {
                // Update mute
                update_set_infraction(doc! { 
                    InfractionField::Offender.as_str(): &user.to_string(),
                    InfractionField::InfractionType.as_str(): &String::from(INFRACTION_MUTE)
                }, 
                infraction_doc(&user, &String::from(INFRACTION_MUTE), &reason, &issued_by, &expiration, &time_stamp)).await;

                if duration != None {
                    msg.channel_id.say(&ctx.http, 
                        &format!("✅ Successfully updated {}'s mute, reason: `{}`, expiring on <t:{}:f>", user.mention(), &reason, &expiration.unwrap())).await?;
                }
                else {
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully updated {}'s mute, reason: {}, lasting `Forever`", user.mention(), &reason)).await?;
                }

            }
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_MUTE)).await;
    }

    Ok(())
}