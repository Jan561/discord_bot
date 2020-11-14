use crate::command;
use crate::permission::PermissionHelper;
use crate::rainbow::utils::player_map;
use crate::rainbow::{Message, Permission};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message as SerenityMessage;

#[command]
async fn unlink(ctx: &Context, msg: &SerenityMessage, mut args: Args) -> CommandResult {
    let cmd = async move {
        let permission = PermissionHelper::new(ctx.clone(), msg.guild_id.unwrap(), msg.author.id);

        permission.check_permission(Permission::Unlink).await?;

        let discord_user = match args.trimmed().current() {
            Some(user) => {
                let user = member!(ctx, msg, user).user;

                permission
                    .check_permission(Permission::UnlinkOtherUser(user.id))
                    .await?;

                user
            }
            None => msg.author.clone(),
        };

        let player_map_lock = player_map(ctx, msg.guild_id.unwrap()).await?;
        let mut player_map = player_map_lock.write().await;

        let removed_player = player_map.remove(&discord_user.id);

        match removed_player {
            Some(p) => {
                if msg.author.id == discord_user.id {
                    Message::Unlinked(p.uplay).send(ctx, msg.channel_id).await?;
                } else {
                    Message::UnlinkedOtherUser(p.uplay, discord_user.id)
                        .send(ctx, msg.channel_id)
                        .await?;
                }
            }
            None => {
                if msg.author.id == discord_user.id {
                    Message::UnlinkedNothing.send(ctx, msg.channel_id).await?;
                } else {
                    Message::UnlinkedNothingOtherUser(discord_user.id)
                        .send(ctx, msg.channel_id)
                        .await?;
                }
            }
        }

        Ok(())
    };

    command::execute(ctx, msg, cmd).await
}
