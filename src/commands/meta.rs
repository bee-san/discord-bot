use gethostname::gethostname;
use log::debug;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[owners_only]
async fn whereami(ctx: &Context, msg: &Message) -> CommandResult {
    debug!("Where in the world am I?");
    let output = format!("{:?}", gethostname());
    debug!("I'm currently runing on {}", &output);
    msg.reply(ctx, output).await?;
    Ok(())
}
