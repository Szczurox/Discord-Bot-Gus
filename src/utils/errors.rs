use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn missing_argument(msg: &Message, ctx: &Context, argument: String) {
    msg.reply(&ctx.http, &format!("❌ You are missing an argument `{}`", argument)).await.expect("MissingArgument Error Error: failed to send");
    println!("MissingArgument Error: channel: {}, user: {}", msg.channel_id, msg.author.id);
    return 
}

pub async fn missing_permission(msg: &Message, ctx: &Context, permission: String) {
    msg.reply(&ctx.http, &format!("❌ You do not have permission to `{}`", permission)).await.expect("MissingPermission Error Error: failed to send");
    println!("MissingPermission Error: channel: {}, user: {}", msg.channel_id, msg.author.id);
    return 
}

pub async fn wrong_argument(msg: &Message, ctx: &Context, argument: String) {
    msg.reply(&ctx.http, &format!("❌ Expected argument `{}`, make sure there are no spelling errors", argument)).await.expect("MissingArgument Error Error: failed to send");
    println!("WrongArgument Error: channel: {}, user: {}", msg.channel_id, msg.author.id);
    return 
}