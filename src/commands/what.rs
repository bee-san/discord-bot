use lemmeknow::Identifier;
use log::{debug, trace};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
async fn what(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let message = args.single::<String>()?;
    trace!("Running lemmeknow");
    trace!("{}", message);

    let user = &msg.author.id;
    let tag_user = format!("👋 <@!{}>", user);

    let mut identifier = Identifier::default();
    identifier.boundaryless = true;
    identifier.min_rarity = 0.1;
    let lemmeknow_result = identifier.identify(&message);
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
