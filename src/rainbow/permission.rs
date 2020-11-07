use crate::permission::is_admin;
use crate::Error;
use crate::Permission as CratePermission;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::id::{GuildId, UserId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    Link,
    LinkOtherUser,
    UnlinkOtherUser,
}

impl Permission {
    pub async fn allowed(self, ctx: &Context, guild: GuildId, user: UserId) -> Result<bool, Error> {
        match self {
            Self::Link => Ok(true),
            Self::LinkOtherUser => is_admin(ctx, guild, user).await,
            Self::UnlinkOtherUser => is_admin(ctx, guild, user).await,
        }
    }
}

pub const LINK_PERMISSION: CratePermission = CratePermission::RainbowPermission(Permission::Link);
pub const LINK_OTHER_USER_PERMISSION: CratePermission =
    CratePermission::RainbowPermission(Permission::LinkOtherUser);
pub const UNLINK_OTHER_USER_PERMISSION: CratePermission =
    CratePermission::RainbowPermission(Permission::UnlinkOtherUser);
