use log::debug;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
async fn ciphey(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let message = args.single::<String>()?;
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
