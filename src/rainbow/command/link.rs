use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
async fn link(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    unimplemented!()
}
