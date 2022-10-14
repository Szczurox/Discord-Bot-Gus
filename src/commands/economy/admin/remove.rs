use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::constants::economy::{Money, Meth};
use crate::utils::args::get_lab_value_change_args;
use crate::utils::errors::wrong_argument;

// Removes specified amount of meth/money from a user 
// Usage: remove [money/meth] [@member / ID] [amount]
#[command]
#[only_in(guilds)]
pub async fn remove(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Parse arguments
    let lab_value_change_args_wrapped = get_lab_value_change_args(ctx, msg, args).await;

    if !lab_value_change_args_wrapped.is_none() {
        // Get arguments into separate variables
        let lab_value_change_args = lab_value_change_args_wrapped.unwrap();
        let item_type = lab_value_change_args.item_type;
        let lab = lab_value_change_args.lab;
        let amount = lab_value_change_args.amount;
        let user = lab_value_change_args.user;
        
        // Check added item type and handle it properly
        if item_type == "money" || item_type == "mo" {
            lab.remove_money(&amount).await;
            msg.channel_id.say(&ctx.http, &format!("✅ Successfully taken away from {} {} dollar(s)", user.mention(), amount)).await?;
        }
        else if item_type == "meth" || item_type == "me" {
            lab.remove_meth(&amount).await;
            msg.channel_id.say(&ctx.http, &format!("✅ Successfully taken away from {} {} pound(s) of meth", user.mention(), amount)).await?;
        }
        else {
            wrong_argument(msg, ctx, String::from("ITEM TYPE")).await;
            return Ok(())
        }
    }
    Ok(())
}