use std::env;

use ares::config::Config;
use ares::perform_cracking;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

use lemmeknow::Identifier;

#[group]
#[commands(ping)]
#[commands(what)]
#[commands(ciphey)]
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
    if lemmeknow_result.is_empty() {
        msg.reply(ctx, "Error: Lemmeknow returned nothing!").await?;
    }
    let mut messages = Vec::new();
    for i in lemmeknow_result {
        println!("i is {:?}", i);
        messages.push(i.data.name);
    }
    let output = messages.join("\n");
    println!("{}", &output);
    msg.reply(ctx, output).await?;
    Ok(())
}

#[command]
async fn ciphey(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.strip_prefix("$ciphey ").unwrap();
    let config = Config::default();
    let result = perform_cracking(message, config);
    if !result.is_some() {
        msg.reply(ctx, "Failed to decode 😭").await?;
    }
    let unwrapped_result = result.unwrap();
    let output = unwrapped_result.text[0].clone();
    let output_path = unwrapped_result
        .path
        .iter()
        .map(|c| c.decoder)
        .collect::<Vec<_>>()
        .join(" → ");
    let output_string = format!(
        "Successfully decoded:\n```\n{}\n```The path is:\n```\n{}\n``` ",
        output, output_path
    );
    msg.reply(ctx, output_string).await?;

    Ok(())
}
