use serenity::model::prelude::*;

// Create member username + # + discriminator string
pub fn get_discord_tag(author: &User) -> String {
    let mut discord_tag: String = author.name.to_owned();
    discord_tag.push_str("#");
    // Discriminator is an unsigned int, so 0s at the beginning (if they exist) have to be added manually
    let discriminator: u16 = author.discriminator;
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