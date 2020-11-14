use log::{error, info};
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
pub async fn after(ctx: &Context, msg: &Message, cmd: &str, result: CommandResult) {
    match result {
        Ok(()) => info!("Processed commanad `{}`", cmd),
        Err(why) => {
            error!("Command `{}` returned error {}", cmd, why);
            let _ = say!(ctx, msg.channel_id, "An error occured. Try again later.");
        }
    }
}
