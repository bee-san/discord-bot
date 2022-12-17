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