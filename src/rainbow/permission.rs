use crate::permission::is_admin;
use crate::Error;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::id::{GuildId, UserId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    Link,
    LinkOtherUser,
}

impl Permission {
    pub async fn allowed(self, ctx: &Context, guild: GuildId, user: UserId) -> Result<bool, Error> {
        match self {
            Self::Link => Ok(true),
            Self::LinkOtherUser => is_admin(ctx, guild, user).await,
        }
    }
}
