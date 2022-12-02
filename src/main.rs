use std::env;

use ares::config::Config;
use ares::perform_cracking;
use log::debug;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::Timestamp;
use serenity::prelude::*;

use lemmeknow::Identifier;
use serenity::utils::Colour;

#[group]
#[commands(ping)]
#[commands(ciphey)]
#[commands(what)]
#[commands(ares)]
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
async fn ares(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.strip_prefix("$ares ").unwrap();
    println!("Trying ciphey");
    println!("The message is {}", message);
    let config = Config::default();
    let result = perform_cracking(message, config);
    if !result.is_some() {
        println!("Failed to decode");
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
    println!("Output is {} and path is {}", output, output_path);

    let user = &msg.author.id;
    let tag_user = format!("👋 <@!{}>", user);

    let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(tag_user).embed(|e| {
                e.title("🛰️ Your text has been successfully cracked.")
                    .field("The plaintext value is:", &output, false)
                    .field("And the decryption path is", &output_path, false)
                    .footer(|f| f.text("http://discord.skerritt.blog"))
                    // Add a timestamp for the current time
                    // This also accepts a rfc3339 Timestamp
                    .timestamp(Timestamp::now())
                    .color(Colour::DARK_GREEN)
            })
        })
        .await;
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    debug!("Pinging");
    msg.reply(ctx, "Pong!").await?;
    println!("Pinging");
    Ok(())
}

#[command]
async fn what(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.strip_prefix("$what ").unwrap();
    println!("Running lemmeknow");
    println!("{}", message);

    let user = &msg.author.id;
    let tag_user = format!("👋 <@!{}>", user);

    let mut identifier = Identifier::default();
    identifier.boundaryless = true;
    identifier.min_rarity = 0.1;
    let lemmeknow_result = identifier.identify(message);
    if lemmeknow_result.is_empty() {
        let _msg = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(&tag_user).embed(|e| {
                    e.title("Failed :cheemsburgar:")
                        .field(
                            "Sadly your text could not be idenfieid :( ",
                            "Try asking in #coded-messages maybe?",
                            false,
                        )
                        .footer(|f| f.text("http://discord.skerritt.blog"))
                        // Add a timestamp for the current time
                        // This also accepts a rfc3339 Timestamp
                        .timestamp(Timestamp::now())
                        .color(Colour::DARK_RED)
                })
            })
            .await?;
    }

    let mut messages = Vec::new();
    for i in lemmeknow_result {
        println!("i is {:?}", i);
        messages.push(i.data.name);
    }
    let output = messages.join("\n");

    let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(&tag_user).embed(|e| {
                e.title("I have identified your text")
                    .field("Your text is one of these:", output, false)
                    .footer(|f| f.text("http://discord.skerritt.blog"))
                    // Add a timestamp for the current time
                    // This also accepts a rfc3339 Timestamp
                    .timestamp(Timestamp::now())
                    .color(Colour::DARK_GREEN)
            })
        })
        .await?;

    Ok(())
}

#[command]
async fn ciphey(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO this is borked
    let message = msg.content.strip_prefix("$ciphey ").unwrap();
    println!("Trying ciphey");
    println!("The message is {}", message);
    let config = Config::default();
    let result = perform_cracking(message, config);
    if !result.is_some() {
        println!("Failed to decode");
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
    println!("Output is {} and path is {}", output, output_path);
    let user = &msg.author.name;
    let tag_uesr = format!("👋 @{}", user);
    let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(tag_uesr).embed(|e| {
                e.title("🛰️ Your text has been successfully cracked.")
                    .field("The plaintext value is:", output, false)
                    .field("And the decryption path is", output_path, false)
                    .footer(|f| f.text("http://discord.skerritt.blog"))
                    // Add a timestamp for the current time
                    // This also accepts a rfc3339 Timestamp
                    .timestamp(Timestamp::now())
                    .color(Colour::DARK_GREEN)
            })
        })
        .await;

    Ok(())
}
