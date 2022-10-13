use mongodb::bson::doc;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::economy::{LabField, Money, Meth};
use crate::utils::economy::get_lab;
use crate::utils::errors::{missing_argument, wrong_argument, lab_doesnt_exist};

// Sets user's amount of meth/money
// Usage: set [money/meth] [@member / ID] [amount]
#[command]
#[only_in(guilds)]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get the guild member for the author
    let member = msg.guild_id.unwrap().member(ctx, msg.author.id).await?;

    if member.permissions(ctx).expect("permissions").contains(Permissions::ADMINISTRATOR) {
        if args.is_empty() {
            missing_argument(msg, ctx, String::from("TYPE [meth/money]")).await;
        } else {
            let added_item_result = args.single::<String>();
            if added_item_result.is_err() {
                wrong_argument(msg, ctx, String::from("TYPE [meth/money]")).await;
                return Ok(())
            }
            let added_item: &str = &added_item_result.unwrap()[..];

            if args.is_empty() {
                missing_argument(msg, ctx, String::from("MEMBER")).await;
                return Ok(())
            }

            // Get user from arguments and throw an error if argument is missing
            let user_result = args.single::<UserId>();
            if user_result.is_err() {
                wrong_argument(msg, ctx, String::from("MEMBER")).await;
                return Ok(())
            }
            let user: UserId = user_result.unwrap();

            if args.is_empty() {
                missing_argument(msg, ctx, String::from("AMOUNT")).await;
                return Ok(())
            }

            let amount_arg = args.single::<i64>();
            if amount_arg.is_err() {
                wrong_argument(msg, ctx, String::from("AMOUNT")).await;
                return Ok(())
            }

            let lab = get_lab(doc! { 
                LabField::UserId.as_str(): &user.to_string(),
            }).await;

            if lab.is_none() {
                lab_doesnt_exist(&msg, &ctx).await;
                return Ok(())
            }

            if added_item == "money" || added_item == "mo" {
                let amount = amount_arg.unwrap();
                lab.unwrap().set_money(&amount).await;
                msg.channel_id.say(&ctx.http, &format!("✅ Successfully set {}'s money to {} dollars", user.mention(), amount)).await?;
            }
            else if added_item == "meth" || added_item == "me" {
                let amount = amount_arg.unwrap();
                lab.unwrap().set_meth(&amount).await;
                msg.channel_id.say(&ctx.http, &format!("✅ Successfully set {}'s meth to {} pounds", user.mention(), amount)).await?;
            }
            else {
                wrong_argument(msg, ctx, String::from("AMOUNT")).await;
                return Ok(())
            }
        }
    }
    Ok(())
}