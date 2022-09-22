// All commands and usage are described using comments above each command's function 
// Using format: command_name [argument] (optional_argument)

mod commands { 
    pub mod ping; 
    pub mod moderation { 
        pub mod kick; 
        pub mod ban; 
    } 
}

mod utils { 
    pub mod erorrs; 
}

use std::env;
use std::sync::Arc;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::framework::standard::{ Configuration };
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use mongodb::{bson::doc, options::ClientOptions};

use crate::commands::ping::*;
use crate::commands::moderation::kick::*;
use crate::commands::moderation::ban::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

// Handler for client events
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

// Group for client commands
#[group]
#[commands(ping, kick, ban)]
struct General;

// Main client function
#[tokio::main]
async fn main() {
    // Load .env file 
    dotenv().ok().expect("Failed to load .env file");

    let connection_string = env::var("MONGODB_CONNECTION_STRING").expect("Expected a token in the environment");

    // Parse onnection string into an options struct
    let mut client_options =
        ClientOptions::parse(connection_string)
            .await.expect("Error parsing connection string");
        
    client_options.app_name = Some("GusBot".to_string());

    // Get a handle to the cluster
    let db_client = mongodb::Client::with_options(client_options).expect("Error getting cluster handle");

    // Ping the server to see if you can connect to the cluster
    db_client
        .database("main-db")
        .run_command(doc! {"ping": 1}, None)
        .await.expect("Error pinging the database");
    println!("Connected to mongodb successfully");

    // List the names of the databases in that cluster
    for db_name in db_client.list_database_names(None, None).await.expect("Error getting databses names") {
        println!("{}", db_name);
    }

    // Get client token from .env
    let token: String = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set client info
    let framework: StandardFramework = StandardFramework::new()
        .configure(|c: &mut Configuration | c.prefix(">"))
        .group(&GENERAL_GROUP);
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
}