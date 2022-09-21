use serenity::model::prelude::*;
use serenity::prelude::*;


pub async fn missing_argument(msg: &Message, ctx: &Context, error: String) {
    msg.reply(&ctx.http, &format!("You are missing an argument `{}`", error)).await.expect("MissingArgument Error Error: failed to send");
    println!("MissingArgument Error: channel: {}, user: {}", msg.channel_id, msg.author.id);
    return 
}

pub async fn missing_permission(msg: &Message, ctx: &Context, error: String) {
    msg.reply(&ctx.http, &format!("You do not have permission to `{}`", error)).await.expect("MissingPermission Error Error: failed to send");
    println!("MissingPermission Error: channel: {}, user: {}", msg.channel_id, msg.author.id);
    return 
}
