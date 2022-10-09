use mongodb::bson::doc;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::economy::LabField;
use crate::utils::economy::{get_lab, lab_setup};

// Start your history with meth cooking
// Usage: start
#[command]
#[only_in(guilds)]
pub async fn start(ctx: &Context, msg: &Message,  _: Args) -> CommandResult {
    let lab = get_lab(doc! { 
        LabField::UserId.as_str(): &msg.author.id.to_string(),
    }).await;
    if lab.is_none() {
        lab_setup(&msg.author.id).await;
        let mut embed = CreateEmbed::default();
        embed.colour(0xC4FFFF);
        embed.title("Meth Cooking Adventure, Begin!");
        embed.description("You decided to become a meth cook for a personal reason.\nYou start by getting an RV that is going to become your first lab.\nThe old guy you took it from has dementia and forgot his RV existed!");
        embed.footer(|f| {
            f.text("Gus Bot by Walter White#0001");
            f.icon_url("https://cdn.discordapp.com/avatars/947853951624183809/2ebd0371852f8ab952f826699e626f0a.png");
            f
        });
        embed.image("https://media.discordapp.net/attachments/999718243654705232/1028745374304833546/unknown.png");
        // Send the infraction log embed
        msg.channel_id.send_message(&ctx.http, |m| {
            m.content(format!("{}", &msg.author.mention()));
            m.embed(|e| {
                *e = embed;
                e
            })
        }).await?;
    }
    else {
        msg.reply_ping(&ctx.http, "❌ You already have a lab").await.expect("Error trying to tell someone that they already have a lab in start command");
    }
    Ok(())
}