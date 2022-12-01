use std::env;

use serde_json::Value;
use serenity::async_trait;
use serenity::futures::future::Map;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use lemmeknow::{Identifier, Data};

#[group]
#[commands(ping)]
#[commands(what)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    println!("Bot received Discord token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn what(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.strip_prefix("$what ").unwrap();
    println!("{}", message);
    let mut identifier = Identifier::default();
    identifier.boundaryless = true;
    identifier.min_rarity = 0.1;
    let lemmeknow_result = identifier.identify(message);
    if lemmeknow_result.is_empty(){
        msg.reply(ctx, "Error: Lemmeknow returned nothing!").await?;
    }
    let mut messages = Vec::new();
    for i in lemmeknow_result{
        println!("i is {:?}", i);
        messages.push(i.data.name);
    }
    let output = messages.join("\n");
    println!("{}", &output);
    msg.reply(ctx, output).await?;
    Ok(())
}

