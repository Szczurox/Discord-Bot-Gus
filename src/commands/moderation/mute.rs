use std::time::Duration;

use mongodb::bson::doc;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::sleep;

use crate::constants::config::{MUTE_ROLE};
use crate::constants::infractions::{INFRACTION_MUTE, InfractionField};
use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::constants::time::{DURATION_TIME, DURATION_TIME_NEVER};
use crate::constants::permissions::PERMISSION_MUTE;
use crate::utils::mongo::{add_infraction, get_infraction, remove_infraction};
use crate::utils::serenity::get_discord_tag;
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

            // Get mute duration time from arguments
            let duration_string: String = args.single::<String>()?;
            // Get time unit (days, months, years, etc.)
            let time_unit: String = duration_string.chars().filter(|c| !c.is_digit(10)).collect();
            let mut duration: String = String::from(DURATION_TIME_NEVER);

            // Check if the duration is specified
            if DURATION_TIME.contains_key(&time_unit[..]) {
                // Get number of time units from the mute duration time string
                let duration_length_string: String = duration_string.chars().filter(|c| c.is_digit(10)).collect();
                let duration_length : u32 = duration_length_string.parse::<u32>().unwrap();

                duration = (DURATION_TIME.get(&time_unit[..]).unwrap() * duration_length).to_string();
            }
            else {
                // Rewing argument if the duration is not specified
                args.rewind();
            }
            
            let reason: String;
            // Check if there is an optional argument "reason" 
            if args.is_empty() {
                reason = String::from("reason not provided");
            }
            else {
                reason = String::from(args.rest());
            }
        
            let time_stamp: u32 = get_time();
            let issued_by: String = get_discord_tag(&msg.author);
            let expiration: String;
            if duration != String::from(DURATION_TIME_NEVER) {
                // Mute end (current time + warn duration)
                expiration = (time_stamp + &duration.parse::<u32>()?).to_string();
            } else {
                expiration = duration.clone();
            }

            add_infraction(&user.to_string(), &String::from(INFRACTION_MUTE), &reason, &issued_by, &expiration, &time_stamp).await;

            // Get a guild member for the user
            let mut member = msg.guild_id.unwrap().member(ctx, user).await?;
            member.add_role(&ctx.http, MUTE_ROLE).await?;

            if duration != String::from(DURATION_TIME_NEVER) {
                msg.channel_id.say(&ctx.http, &format!("✅ Successfully muted {} for `{}`, expiring on <t:{}:f>", user.mention(), &reason, &expiration)).await?;

                sleep(Duration::from_secs(duration.parse::<u64>()?)).await;
                
                // Remove mute from the infraction log
                let infraction = get_infraction(doc! { 
                    InfractionField::Offender.as_str(): &user.to_string(),
                    InfractionField::IssuedBy.as_str(): &issued_by, 
                    InfractionField::InfractionType.as_str(): &String::from(INFRACTION_MUTE), 
                    InfractionField::CreationDate.as_str(): &time_stamp 
                }).await.unwrap();

                remove_infraction(infraction._id).await;

                member.remove_role(&ctx.http, MUTE_ROLE).await?;
            }
            else {
                msg.channel_id.say(&ctx.http, &format!("✅ Successfully muted {} for `{}`, lasting `Forever`", user.mention(), &reason)).await?;
            }
        }
    } else {
        missing_permission(msg, ctx, String::from(PERMISSION_MUTE)).await;
    }

    Ok(())
}