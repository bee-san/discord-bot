use std::collections::HashMap;
use std::env;

use ares::config::Config;
use ares::perform_cracking;
use gethostname::gethostname;
use log::{debug, error, trace};
use serde::Deserialize;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::Timestamp;
use serenity::prelude::*;

use lemmeknow::Identifier;
use serenity::utils::Colour;

#[group]
#[commands(what, ares, ping, whereami, help, ciphey, sth)]
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
    debug!("Received discord token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ares(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.strip_prefix("$ares ").unwrap();
    let user = &msg.author.id;
    let tag_user = format!("👋 <@!{}>", user);

    let mut to_decode: String = "Default".to_string();
    if message.contains("pastebin") {
        if !message.contains("/raw/") {
            let _msg = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(&tag_user).embed(|e| {
                    e.title("Failed 😢")
                        .field(
                            "Could not open your Pastebin, it needs to be the raw data.",
                            "Please add /raw/ to your pastebin like https://pastebin.com/raw/37VuHzqa or by clicking 'raw' on the paste.",
                            false,
                        )
                        .footer(|f| f.text("http://discord.skerritt.blog"))
                        // Add a timestamp for the current time
                        // This also accepts a rfc3339 Timestamp
                        .url("https://github.com/bee-san/ares")
                        .timestamp(Timestamp::now())
                        .color(Colour::DARK_RED)
                })
            })
            .await?;
        } else {
            println!("Pastebin is raw");
            // Pastebin is raw
            let resp = reqwest::get(message).await?.text().await?;
            to_decode = resp;
        }
    } else {
        to_decode = message.to_string();
    }

    trace!("Trying Ciphey");
    trace!("The message is {}", &to_decode);
    let mut config = Config::default();
    // 10 seconds because the bot is slow
    config.timeout = 1;
    let result = perform_cracking(&to_decode, config);
    if result.is_none() {
        trace!("Ares is returning something....");
        let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(&tag_user).embed(|e| {
                e.title("Failed 😢")
                    .field(
                        "Sadly your text could not be decoded 🙈 ",
                        "Try using the CLI tool or visiting #coded-messages https://discord.com/channels/754001738184392704/829065151851528243",
                        false,
                    )
                    .footer(|f| f.text("http://discord.skerritt.blog"))
                    // Add a timestamp for the current time
                    // This also accepts a rfc3339 Timestamp
                    .url("https://github.com/bee-san/ares")
                    .timestamp(Timestamp::now())
                    .color(Colour::DARK_RED)
            })
        })
        .await?;
    }
    let unwrapped_result = result.unwrap();
    trace!("Decoder is unwrapped");
    let output = unwrapped_result.text[0].clone();
    let output_path = unwrapped_result
        .path
        .iter()
        .map(|c| c.decoder)
        .collect::<Vec<_>>()
        .join(" → ");
    debug!("Output is {} and path is {}", output, output_path);

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
    Ok(())
}

#[command]
async fn whereami(ctx: &Context, msg: &Message) -> CommandResult {
    debug!("Where in the world am I?");
    let output = format!("{:?}", gethostname());
    debug!("I'm currently runing on {}", &output);
    msg.reply(ctx, output).await?;
    Ok(())
}

#[command]
async fn what(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.strip_prefix("$what ").unwrap();
    trace!("Running lemmeknow");
    trace!("{}", message);

    let user = &msg.author.id;
    let tag_user = format!("👋 <@!{}>", user);

    let mut identifier = Identifier::default();
    identifier.boundaryless = true;
    identifier.min_rarity = 0.1;
    let lemmeknow_result = identifier.identify(message);
    if lemmeknow_result.is_empty() {
        debug!("Lemmeknow is returing failed state");
        let _msg = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(&tag_user).embed(|e| {
                    e.title("Failed 😿")
                        .field(
                            "Sadly your text could not be identified 🙈 ",
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
        messages.push(i.data.name);
    }
    let output = messages.join("\n");
    debug!("Lemmeknow is returing found on the string {}", &output);

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
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let message: &str = "
👋 The ultimate hacking bot
Brought to you by the folks over at https://discord.com/invite/zYTM3rZM4T

```
$ares aGVsbG8=
```

Decodes the text aGVsbG8= with Ares, the next generation of Ciphey <http://github.com/bee-san/Ares>

```
$ares https://pastebin.com/raw/05umSkne
```

If your file is too large to copy & paste you can upload it to pastebin, get the raw link and Ares will decode it.

```
$what 192.168.0.1
```

Run Lemmeknow / PyWhat on the input to identify it <https://github.com/swanandx/lemmeknow>

```
$ping
```

PONG!

```
$sth 5f4dcc3b5aa765d61d8327deb882cf99
```

Runs Search-That-Hash on the hash.
    ";

    msg.reply(ctx, message).await?;
    Ok(())
}

#[command]
async fn ciphey(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.strip_prefix("$ciphey ").unwrap();
    let user = &msg.author.id;
    let tag_user = format!("👋 <@!{}>", user);

    // let ciphey_api_url = env::var("CIPHEY_URL").expect("Ciphey URL");
    let ciphey_api_url = format!(
        "https://pl8u5p7v00.execute-api.us-east-2.amazonaws.com/default/ciphey_lambda_api?ctext={}",
        message
    );

    // Create a new reqwest client
    let client = reqwest::Client::new();

    // Make a GET request to the specified URL
    let response = client.get(ciphey_api_url).send().await?;

    // Print the response status
    debug!("Response status: {}", response.status());

    // Read the response body
    let body = response.text().await?;

    if body == "{\"message\": \"Internal server error\"}" {
        let _msg = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(&tag_user).embed(|e| {
                    e.title("😭 Error: Your text could not be decoded")
                        .field(
                            "You have some alternatives",
                            "Use $ares to use Ares, use the Ciphey CLI or use the Ares CLI.",
                            false,
                        )
                        .footer(|f| f.text("http://discord.skerritt.blog"))
                        // Add a timestamp for the current time
                        // This also accepts a rfc3339 Timestamp
                        .timestamp(Timestamp::now())
                        .color(Colour::RED)
                })
            })
            .await?;
    } else {
        let _msg = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(&tag_user).embed(|e| {
                    e.title("🥳 Your text has been decoded")
                        .field("The plaintext is:", body.trim_matches('"'), false)
                        .footer(|f| f.text("http://discord.skerritt.blog"))
                        // Add a timestamp for the current time
                        // This also accepts a rfc3339 Timestamp
                        .timestamp(Timestamp::now())
                        .color(Colour::DARK_GREEN)
                })
            })
            .await?;
    }
    Ok(())
}

#[command]
async fn sth(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Trying STH");
    let message = msg.content.strip_prefix("$sth ").unwrap();
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
        Plaintext: String,
        Type: String,
    }


    let mut data = HashMap::new();
    data.insert("Hash", [&hash]);

    let client = reqwest::Client::new();
    let resp = client
        .get("https://av5b81zg3k.execute-api.us-east-2.amazonaws.com/prod/lookup")
        .json(&data)
        .send()
        .await.unwrap();
    
        let mut text: Data = resp.json().await.unwrap();
    let data = text.body.remove(message).unwrap();
    let output = data.Plaintext;
    let output_type = data.Type;

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