use std::env;
use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
// use aria_client::Client;
// use std::sync::Arc;

// add a music bot next soon tm use above later

struct Handler;
// system that responds to hardcoded commands like !ping
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content == "!hello" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello there!").await {
                println!("Error sending message: {why:?}");
            }            
        } else if msg.content == "!status" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Bot is online.").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}


#[tokio::main]
async fn main() {
    // Load the .env file
    dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
