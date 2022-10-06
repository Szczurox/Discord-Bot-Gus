use std::{sync::Arc};

use serenity::{model::prelude::*, http::Http, Error, framework::standard::Args};

use crate::constants::{config::GUILD_ID, time::DURATION_TIME};

// Create member username + # + discriminator string
pub fn get_discord_tag(user: &User) -> String {
    let mut discord_tag: String = user.name.to_owned();
    discord_tag.push_str("#");
    // Discriminator is an unsigned int, so 0s at the beginning (if they exist) have to be added manually
    let discriminator: u16 = user.discriminator;
    if discriminator < 1000 {
        discord_tag.push_str("0");
    }
    if discriminator < 100 {
        discord_tag.push_str("0");
    }
    if discriminator < 10 {
        discord_tag.push_str("0");
    }
    discord_tag.push_str(&discriminator.to_string());
    
    discord_tag
}

// Remove role from a member
pub async fn remove_role(http: &Arc<Http>, user_id: UserId, role_id: u64) -> Result<(), Error> {
    let mut member = GuildId::from(GUILD_ID).member(http, user_id).await.unwrap();
    let result = member.remove_role(&http, role_id).await;
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    Ok(())
}

// Add role to a member
pub async fn add_role(http: &Arc<Http>, user_id: UserId, role_id: u64) -> Result<(), Error> {
    let mut member = GuildId::from(GUILD_ID).member(http, user_id).await.unwrap();
    let result = member.add_role(&http, role_id).await;
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    Ok(())
}

// Check if member has a role
pub async fn check_role(http: &Arc<Http>, user_id: UserId, role_id: u64) -> bool {
    let member = GuildId::from(GUILD_ID).member(http, user_id).await.unwrap();
    member.roles.contains(&RoleId::from(role_id))
}

// Get a duration from some command arguments
pub fn get_duration_from_args(args: &mut Args) -> Option<u32> {
    if !args.is_empty() {
        // Get mute duration time from arguments
        let duration_string: String = args.single::<String>().expect("Error getting duration string");
        // Get time unit (days, months, years, etc.)
        let time_unit: String = duration_string.chars().filter(|c| !c.is_digit(10)).collect();

        // Check if the duration is specified
        if DURATION_TIME.contains_key(&time_unit[..]) {
            // Get number of time units from the mute duration time string
            let duration_length_string: String = duration_string.chars().filter(|c| c.is_digit(10)).collect();
            let duration_length : u32 = duration_length_string.parse::<u32>().unwrap();

            return Some(DURATION_TIME.get(&time_unit[..]).unwrap() * duration_length);
        }
        else {
            args.rewind();
        }
    }

    None
}

// Get a reason from some command arguments
pub fn get_reason_from_args(args: &mut Args) -> String { 
    if args.is_empty() {
        return String::from("reason not provided");
    }
    String::from(args.rest())
}