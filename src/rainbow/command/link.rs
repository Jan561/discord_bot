use crate::command::Error as CommandError;
use crate::permission::{check_permission, PermissionFacility};
use crate::rainbow::permission::{LINK_OTHER_USER_PERMISSION, LINK_PERMISSION};
use crate::Error;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
async fn link(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let permission = PermissionFacility::new(ctx.clone(), msg.guild_id.unwrap(), msg.author.id);

    permission.check_permission(LINK_PERMISSION)??;

    let uplay = args
        .trimmed()
        .current()
        .ok_or(CommandError::ArgumentMissing(Some(
            "Uplay-Account".to_string(),
        )))?;

    let discord_user = match args.advance().current() {
        Some(user) => {
            permission.check_permission(LINK_OTHER_USER_PERMISSION)??;

            // FIXME
            let _ = msg
                .guild(&ctx.cache)
                .await
                .ok_or(Error::CacheGuildMissing(msg.guild_id.unwrap()))?
                .member_named(user);
        }
        None => &msg.author,
    };

    Ok(())
}
