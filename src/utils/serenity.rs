use std::sync::Arc;

use serenity::{model::prelude::*, http::Http, Error};

use crate::constants::config::GUILD_ID;

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

pub async fn remove_role(http: &Arc<Http>, user_id: UserId, role_id: u64) -> Result<(), Error> {
    let mut member = GuildId::from(GUILD_ID).member(http, user_id).await.unwrap();
    let result = member.remove_role(&http, role_id).await;
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    Ok(())
}

pub async fn add_role(http: &Arc<Http>, user_id: UserId, role_id: u64) -> Result<(), Error> {
    let mut member = GuildId::from(GUILD_ID).member(http, user_id).await.unwrap();
    let result = member.add_role(&http, role_id).await;
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    Ok(())
}