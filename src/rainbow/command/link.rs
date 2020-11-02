use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
async fn link(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    unimplemented!()
}
