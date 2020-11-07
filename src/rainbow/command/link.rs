use crate::command::Error as CommandError;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
async fn link(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let uplay = args
        .trimmed()
        .current()
        .ok_or(CommandError::ArgumentMissing(Some(
            "Uplay-Accont".to_string(),
        )))?;
}
