use crate::rainbow::Permission as RainbowPermission;
use crate::Error;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::id::{GuildId, UserId};
use serenity::model::Permissions;

#[async_trait]
pub trait Entity {
    async fn has_permission(
        &self,
        ctx: &Context,
        guild: GuildId,
        permission: Permission,
    ) -> Result<bool, Error>;
}

#[async_trait]
impl Entity for UserId {
    async fn has_permission(
        &self,
        ctx: &Context,
        guild: GuildId,
        permission: Permission,
    ) -> Result<bool, Error> {
        permission.allowed(ctx, guild, *self).await
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Permission {
    RainbowPermission(RainbowPermission),
}

impl Permission {
    pub fn allowed(self, ctx: &Context, guild: GuildId, user: UserId) -> Result<bool, Error> {
        match self {
            Self::RainbowPermission(p) => p.allowed(ctx, guild, user),
        }
    }
}

pub async fn owner(guild: GuildId) -> Result<UserId, Error> {
    Ok(guild
        .to_guild_cached(&ctx.cache)
        .await
        .ok_or(Error::CacheGuildMissing(guild))?
        .owner_id)
}

pub async fn is_admin(ctx: &Context, guild: GuildId, user: UserId) -> Result<bool, Error> {
    if user == owner(guild)? {
        return Ok(true);
    }

    let member = guild.member(&ctx.http, user).await?;
    for &role in member.roles.iter() {
        if role
            .to_role_cached(&ctx.cache)
            .await
            .ok_or(Error::CacheRoleMissing(role))?
            .has_permission(Permissions::ADMINISTRATOR)
        {
            return Ok(true);
        }
    }

    Ok(false)
}
