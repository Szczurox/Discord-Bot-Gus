// All commands and usage are described using comments above each command's function 
// Using format: command_name [argument <argument_syntax>] (optional_argument)

mod commands { 
    pub mod ping; 
    pub mod moderation { 
        pub mod kick; 
        pub mod ban; 
        pub mod unban;
        pub mod mute; 
        pub mod unmute;
        pub mod warn; 
        pub mod search;
        pub mod removewarn;
    } 
    pub mod economy {
        pub mod start;
        pub mod admin {
            pub mod add;
            pub mod remove;
            pub mod set;
        }
    }
}

mod utils { 
    pub mod args;
    pub mod errors; 
    pub mod mongo;
    pub mod serenity;
    pub mod time;
    pub mod infractions;
    pub mod economy;
}

mod events {
    pub mod event_handler;
}

mod constants {
    pub mod time;
    pub mod permissions;
    pub mod infractions;
    pub mod config;
    pub mod economy;
}

use std::sync::Arc;

use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::framework::standard::{ Configuration };
use serenity::prelude::*;

use shuttle_service::error::CustomError;
use shuttle_service::SecretStore;

use sqlx::PgPool;

use mongodb::bson::doc;

use crate::commands::ping::*;
use crate::commands::moderation::kick::*;
use crate::commands::moderation::ban::*;
use crate::commands::moderation::unban::*;
use crate::commands::moderation::mute::*;
use crate::commands::moderation::unmute::*;
use crate::commands::moderation::warn::*;
use crate::commands::moderation::search::*;
use crate::commands::moderation::removewarn::*;
use crate::commands::economy::start::*;
use crate::commands::economy::admin::add::*;
use crate::commands::economy::admin::remove::*;
use crate::commands::economy::admin::set::*;

use crate::events::event_handler::Handler;

use crate::utils::mongo::{init_mongo_client, get_mongo_db};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// Group for client commands
#[group]
#[commands(ping)]
struct General;

// Group for moderation commands
#[group]
#[commands(kick, ban, unban, mute, unmute, warn, search, removewarn)]
struct Moderation;

// Group for economy commands
#[group]
#[commands(start)]
struct Economy;

// Group for economy admin commands
#[group]
#[commands(add, remove, set)]
struct EconomyAdmin;

// Main client function
#[shuttle_service::main]
async fn serenity(#[shared::Postgres] pool: PgPool) -> shuttle_service::ShuttleSerenity {
    // Get connection string from 'Secrets.toml'
    let connection_string: String = pool
    .get_secret("MONGODB_CONNECTION_STRING")
    .await
    .map_err(CustomError::new)?;
    
    init_mongo_client(connection_string).await;

    let db = get_mongo_db().unwrap();

    // Ping the server to see if you can connect to the cluster
    db.run_command(doc! {"ping": 1}, None).await.expect("Error pinging the database");
    println!("Connected to mongodb successfully");

    // Get client token from 'Secrets.toml'
    let token = pool
        .get_secret("DISCORD_TOKEN")
        .await
        .map_err(CustomError::new)?;

    // Set client info
    let framework: StandardFramework = StandardFramework::new()
        .configure(|c: &mut Configuration | c.prefix("."))
        .group(&GENERAL_GROUP)
        .group(&MODERATION_GROUP)
        .group(&ECONOMY_GROUP)
        .group(&ECONOMYADMIN_GROUP);
    let intents: GatewayIntents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Set up shard manager for commands spreaded betwen public functions in other modules
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }
    
    let shard_manager = client.shard_manager.clone();

    // Wait for ctrl+c signal, shut down if signal is received
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    // If there is an error, print it
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(client)
}