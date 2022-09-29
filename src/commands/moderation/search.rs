use mongodb::{
    bson::doc
};
use serenity::builder::CreateEmbed;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::infractions::Infraction;
use crate::utils::errors::{missing_permission, wrong_argument};
use crate::constants::permissions::PERMISSION_SEARCH;
use crate::utils::mongo::{get_mongo_db};
use crate::utils::serenity::get_discord_tag;

// Search member's infraction log, you can search other members only if you have mod perms
// Usage: search (@member / ID)
#[command]
#[only_in(guilds)]
pub async fn search(ctx: &Context, msg: &Message,  mut args: Args) -> CommandResult {
    // Searched user
    let user_id: UserId;

    // If the searched user is not specified set the searched user to the message author
    if args.is_empty() {
        user_id = msg.author.id;
    } 
    else {
        // Get guild member for author
        let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;
        // Set the searched user to the one chosen in the arguments
        let user_id_result = args.single::<UserId>();
        if user_id_result.is_err() {
            wrong_argument(msg, ctx, String::from("MEMBER")).await;
            return Ok(())
        }
        user_id = user_id_result.unwrap();
        // Check if member has mod permissions or is the searched user
        if !member.permissions(ctx).expect("permissions").contains(Permissions::MODERATE_MEMBERS) && msg.author.id != user_id {
            missing_permission(msg, ctx, String::from(PERMISSION_SEARCH)).await;
            return Ok(())
        }
    }

    // Get database handle
    let db = get_mongo_db().unwrap();
    // Get infractions collection handle
    let collection = db.collection::<Infraction>("infractions");
    // Get all infraction logs from a user
    let mut infraction_log = collection.find(
        doc! {
            "offender": &user_id.to_string()
        }, 
        None,
    ).await?;

    // Get searched user's tag
    let searched_user: User = user_id.to_user(ctx).await.unwrap();
    let searched_user_tag: String = get_discord_tag(&searched_user);
    // Create embed
    let mut embed = CreateEmbed::default();
    embed.colour(0xC4FFFF);
    embed.title(format!("{}'s Infraction log", searched_user_tag));
    embed.footer(|f| {
        f.text("Gus Bot by Walter White#0001");
        f.icon_url("https://cdn.discordapp.com/avatars/947853951624183809/2ebd0371852f8ab952f826699e626f0a.png");
        f
    });
    // Traverse through searched user's infraction log and get all infraction's 
    while infraction_log.advance().await? {
        let infraction: Infraction = infraction_log.deserialize_current()?;
        let infraction_expiration_string;
        if infraction.expiration_date == None {
            infraction_expiration_string = String::from("Never");
        }
        else {
            infraction_expiration_string = format!("<t:{}:f>", infraction.expiration_date.unwrap().to_string())
        }
        // Add infractions as embed fields
        embed.field(format!("**{}**", infraction.infraction_type), 
            format!("**Reason:** `{}`\n**Issued By:** {}\n**Expiring:** {}\n**Created:** <t:{}:f>\n**ID:** {}", 
                    infraction.reason, infraction.issued_by, infraction_expiration_string, infraction.creation_date, infraction._id), true);
    }

    // Send the infraction log embed
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            *e = embed;
            e
        })
    }).await?;

    Ok(())
}