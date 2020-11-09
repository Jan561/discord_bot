use crate::rainbow::model::Uplay;
use crate::Error;
use serenity::client::Context;
use serenity::model::channel::Message as SerenityMessage;
use serenity::model::id::{ChannelId, UserId};

#[derive(Clone, Debug)]
pub enum Message {
    Linked(Uplay),
    LinkedOtherUser(Uplay, UserId),
    Unlinked(Uplay),
    UnlinkedOtherUser(Uplay, UserId),
}

impl Message {
    pub async fn send(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
    ) -> Result<SerenityMessage, Error> {
        let message = match self {
            Self::Linked(uplay) => say!(ctx, channel_id, "Linked `{}` to your discord.", uplay)?,
            Self::LinkedOtherUser(uplay, user) => {
                let user = user.to_user(&ctx.http).await?;
                say!(ctx, channel_id, "Linked `{}` to `{}`.", uplay, user.tag())?
            }
            Self::Unlinked(uplay) => {
                say!(ctx, channel_id, "Unlinked `{}` from your discord.", uplay)?
            }
            Self::UnlinkedOtherUser(uplay, user) => {
                let user = user.to_user(&ctx.http).await?;
                say!(
                    ctx,
                    channel_id,
                    "Unlinked `{}` from `{}`.",
                    uplay,
                    user.tag()
                )?
            }
        };

        Ok(message)
    }
}
