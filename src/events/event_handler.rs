use serenity::async_trait;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::utils::infractions::infraction_expiration_coroutine;

pub struct Handler;

// Handler for client events
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
        
        tokio::spawn(async move {
            infraction_expiration_coroutine(&ctx.http).await;
        });
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}
