use ares::config::Config;
use ares::perform_cracking;
use log::{debug, trace};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
async fn ares(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let message = args.single::<String>()?;
    // Gets the next argument which is timeout
    // if the argument isn't included default to 10
    let argument_timeout = match args.single::<String>() {
        Ok(x) => {
            // The argument is supposed to look like timeout=5
            println!("The timeout argument looks like {}", &x);
            let y = x.strip_prefix("timeout=").expect("Error: The user did not provide the argument in the right way");
            y.to_owned()
        },
        Err(_) => "10".to_owned(),
    };
    println!("The timeout is {}", &argument_timeout);
    let user = &msg.author.id;
    let tag_user = format!("👋 <@!{}>", user);

    let mut to_decode: String = message.to_string();
    if message.contains("pastebin") {
        println!("Message containes Pastebin link");
        if !message.contains("/raw/") {
            println!("Message does not contain /raw/ pastebin link");
            send_pastebin_message(ctx, msg, &tag_user).await;
        } else {
            println!("Pastebin is raw");
            // Pastebin is raw
            let resp = reqwest::get(message).await?.text().await?;
            to_decode = resp;
        }
    } else {
        to_decode = message.to_string();
    }

    trace!("Trying Ares");
    trace!("The message is {}", to_decode);
    let mut config = Config::default();
    // 10 seconds because the bot is slow
    config.timeout = str::parse(&argument_timeout).unwrap();
    println!("The config timeout set is {}", &config.timeout);
    // TODO if I add a setter method to ares we can disallow timeouts in negatives or too high etc
    if config.timeout > 30 {
        panic!("Cannot make timeout that long");
    }
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
       return Ok(());
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

    sucess_message(ctx, msg, &tag_user, &output, &output_path).await;
    Ok(())
}

/// Fail if the pastebin does not look right
async fn send_pastebin_message(ctx: &Context, msg: &Message, tag_user: &str) {
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
    .await.expect("Could not send pastebin failure message");
}

async fn sucess_message(ctx: &Context, msg: &Message, tag_user: &str, output: &str, output_path: &str) {
    let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(tag_user).embed(|e| {
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
        .await.expect("Could not send success ares message");
}