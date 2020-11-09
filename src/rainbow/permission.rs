use crate::permission::is_admin;
use crate::Error;
use crate::Permission as CratePermission;
use serenity::client::Context;
use serenity::model::id::{GuildId, UserId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    Link,
    LinkOtherUser(UserId),
    Unlink,
    UnlinkOtherUser(UserId),
}

impl Permission {
    pub async fn allowed(
        &self,
        ctx: &Context,
        guild: GuildId,
        user: UserId,
    ) -> Result<bool, Error> {
        match self {
            Self::Link => Ok(true),
            Self::LinkOtherUser(_) => is_admin(ctx, guild, user).await,
            Self::Unlink => Ok(true),
            Self::UnlinkOtherUser(_) => is_admin(ctx, guild, user).await,
        }
    }
}

impl From<Permission> for CratePermission {
    fn from(perm: Permission) -> Self {
        Self::RainbowPermission(perm)
    }
}
