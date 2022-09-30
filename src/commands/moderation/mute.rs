use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::config::{MUTE_ROLE};
use crate::constants::infractions::{INFRACTION_MUTE};
use crate::utils::errors::{missing_argument, missing_permission, wrong_argument};
use crate::constants::time::{DURATION_TIME};
use crate::constants::permissions::PERMISSION_MUTE;
use crate::utils::infractions::{add_infraction};
use crate::utils::serenity::{get_discord_tag, add_role};
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
            let duration: Option<u32>;

            // Check if the duration is specified
            if DURATION_TIME.contains_key(&time_unit[..]) {
                // Get number of time units from the mute duration time string
                let duration_length_string: String = duration_string.chars().filter(|c| c.is_digit(10)).collect();
                let duration_length : u32 = duration_length_string.parse::<u32>().unwrap();

                duration = Some(DURATION_TIME.get(&time_unit[..]).unwrap() * duration_length);
            }
            else {
                duration = None;
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
            let expiration: Option<u32>;
            if duration != None {
                // Mute end (current time + warn duration)
                expiration = Some(time_stamp + &duration.unwrap());
            } else {
                expiration = None;
            }

            add_infraction(&user, &String::from(INFRACTION_MUTE), &reason, &issued_by, &expiration, &time_stamp).await;

            add_role(&ctx.http, user, MUTE_ROLE).await?;

            if duration != None {
                msg.channel_id.say(&ctx.http, &format!("✅ Successfully muted {} for `{}`, expiring on <t:{}:f>", user.mention(), &reason, &expiration.unwrap())).await?;
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