use super::Error as RainbowError;
use crate::command::{self, Error as CommandError};
use crate::permission::PermissionHelper;
use crate::rainbow::model::{Player, Uplay};
use crate::rainbow::utils::player_map;
use crate::rainbow::{Message, Permission};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message as SerenityMessage;

#[command]
async fn link(ctx: &Context, msg: &SerenityMessage, mut args: Args) -> CommandResult {
    let cmd = async move {
        let permission = PermissionHelper::new(ctx.clone(), msg.guild_id.unwrap(), msg.author.id);

        permission.check_permission(Permission::Link).await?;

        let uplay = args
            .trimmed()
            .current()
            .ok_or(CommandError::ArgumentMissing("Uplay-Account".to_string()))?
            .to_string();
        let uplay = Uplay(uplay);

        let discord_user = match args.advance().current() {
            Some(user) => {
                let user = member!(ctx, msg, user).user;

                permission
                    .check_permission(Permission::LinkOtherUser(user.id))
                    .await?;

                user
            }
            None => msg.author.clone(),
        };

        let player_map_lock = player_map(ctx, msg.guild_id.unwrap()).await?;
        let mut player_map = player_map_lock.write().await;

        // Check if uplay user is already taken
        let existing = player_map
            .iter()
            .find(|&(_, p)| &p.uplay == &uplay)
            .map(|(&u, p)| (u, p.clone()));
        if existing.is_some() {
            return Err(CommandError::from(RainbowError::UplayTaken(uplay)).into());
        }

        let old_player = player_map.insert(
            discord_user.id,
            Player {
                uplay: uplay.clone(),
            },
        );

        if let Some(old) = old_player {
            if discord_user.id == msg.author.id {
                Message::Unlinked(old.uplay)
                    .send(ctx, msg.channel_id)
                    .await?;
            } else {
                Message::UnlinkedOtherUser(old.uplay, discord_user.id)
                    .send(ctx, msg.channel_id)
                    .await?;
            }
        }

        if discord_user.id == msg.author.id {
            Message::Linked(uplay).send(ctx, msg.channel_id).await?;
        } else {
            Message::LinkedOtherUser(uplay, discord_user.id)
                .send(ctx, msg.channel_id)
                .await?;
        }

        Ok(())
    };

    command::execute(ctx, msg, cmd).await
}
