use mongodb::bson::doc;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::infractions::{INFRACTION_BAN, InfractionField};
use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::utils::infractions::{add_infraction, update_set_infraction, infraction_doc, get_infraction};
use crate::utils::serenity::{get_discord_tag, get_reason_from_args, get_duration_from_args};
use crate::utils::time::{get_time};
use crate::constants::permissions::PERMISSION_BAN;

// Ban a member from a guild
// Usage: ban [@member / ID] (reason) 
#[command]
#[only_in(guilds)]
pub async fn ban(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
    // Get the guild member for the author
    let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;

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
            
            let duration: Option<u32> = get_duration_from_args(&mut args);

            let reason: String = get_reason_from_args(&mut args);

            let time_stamp: u32 = get_time();
            let expiration: Option<u32>;
            if duration != None {
                // Mute end (current time + warn duration)
                expiration = Some(time_stamp + &duration.unwrap());
            } else {
                expiration = None;
            }
            // Add the ban to the infractions log
            let issued_by: String = get_discord_tag(&msg.author);
            let infraction = get_infraction(doc! { 
                InfractionField::Offender.as_str(): &user.to_string(),
                InfractionField::InfractionType.as_str(): &String::from(INFRACTION_BAN)
            }).await;
            if infraction.is_none() {
                // Ban a member from the guild
                let result = msg.guild_id.unwrap().ban_with_reason(&ctx.http, user, 0, &reason).await;
                if !result.is_err() {
                    add_infraction(&user, &String::from(INFRACTION_BAN), &reason, &issued_by, &expiration, &time_stamp).await;
                    // Send a message confirming the ban
                    if &reason[..] == "reason not provided" {
                        msg.channel_id.say(&ctx.http, &format!("✅ Successfully banned {}", user.mention())).await?;
                    } 
                    else {
                        msg.channel_id.say(&ctx.http, &format!("✅ Successfully banned {} for `{}`", user.mention(), &reason)).await?;
                    }
                } else {
                    missing_permission(msg, ctx, String::from(PERMISSION_BAN)).await;
                }
            }
            else {
                // Update ban
                update_set_infraction(doc! { 
                    InfractionField::Offender.as_str(): &user.to_string(),
                    InfractionField::InfractionType.as_str(): &String::from(INFRACTION_BAN)
                }, 
                infraction_doc(&user, &String::from(INFRACTION_BAN), &reason, &issued_by, &expiration, &time_stamp)).await;

                if duration != None {
                    msg.channel_id.say(&ctx.http, 
                        &format!("✅ Successfully updated {}'s ban, reason: `{}`, expiring on <t:{}:f>", user.mention(), &reason, &expiration.unwrap())).await?;
                }
                else {
                    msg.channel_id.say(&ctx.http, &format!("✅ Successfully updated {}'s ban, reason: {}, lasting `Forever`", user.mention(), &reason)).await?;
                }
            }
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_BAN)).await;
    }

    Ok(())
}