use log::info;
use serenity::client::Context;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[hook]
pub async fn before(_: &Context, msg: &Message, cmd: &str) -> bool {
    info!("Got command {} from {}", cmd, msg.author.tag());

    true
}

#[hook]
pub async fn after(_ctx: &Context, _msg: &Message, _cmd: &str, _result: CommandResult) {}
