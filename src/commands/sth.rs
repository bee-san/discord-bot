

use serde::Deserialize;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;
use std::collections::HashMap;

#[command]
async fn sth(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("Trying STH");
    let message = &args.single::<String>()?;
    let user = &msg.author.id;
    let tag_user = format!("👋 <@!{}>", user);
    println!("Trying STH");

    // Create a new reqwest client
    let hash = message;

    #[derive(Deserialize)]
    struct Data {
        body: std::collections::HashMap<String, Body>,
    }

    #[derive(Deserialize)]
    struct Body {
        plaintext: String,
        hashtype: String,
    }

    let mut data = HashMap::new();
    data.insert("Hash", [&hash]);

    let client = reqwest::Client::new();
    let resp = client
        .get("https://av5b81zg3k.execute-api.us-east-2.amazonaws.com/prod/lookup")
        .json(&data)
        .send()
        .await
        .unwrap();

    let mut text: Data = resp.json().await.unwrap();
    let data = text.body.remove(message).unwrap();
    let output = data.plaintext;
    let output_type = data.hashtype;

    let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(&tag_user).embed(|e| {
                e.title("🥳 Your text has been de-hashed!")
                    .field("The plaintext is: ", output, false)
                    .field("And the hash type is: ", output_type, false)
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
