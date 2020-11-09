use crate::command::Error as CommandError;
use crate::error::GeneralError;
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
        permission: impl Into<Permission> + Send + 'async_trait,
    ) -> Result<bool, Error>;
}

#[async_trait]
impl Entity for UserId {
    async fn has_permission(
        &self,
        ctx: &Context,
        guild: GuildId,
        permission: impl Into<Permission> + Send + 'async_trait,
    ) -> Result<bool, Error> {
        has_permission(ctx, guild, *self, permission).await
    }
}

#[derive(Clone, Debug)]
pub enum Permission {
    RainbowPermission(RainbowPermission),
}

impl Permission {
    pub async fn allowed(
        &self,
        ctx: &Context,
        guild: GuildId,
        user: UserId,
    ) -> Result<bool, Error> {
        match self {
            Self::RainbowPermission(p) => p.allowed(ctx, guild, user).await,
        }
    }
}

pub async fn guild_owner(ctx: &Context, guild: GuildId) -> Result<UserId, Error> {
    Ok(guild
        .to_guild_cached(&ctx.cache)
        .await
        .ok_or(Error::CacheGuildMissing(guild))?
        .owner_id)
}

pub async fn is_admin(ctx: &Context, guild: GuildId, user: UserId) -> Result<bool, Error> {
    if user == guild_owner(ctx, guild).await? {
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

pub async fn has_permission(
    ctx: &Context,
    guild: GuildId,
    user: UserId,
    permission: impl Into<Permission> + Send,
) -> Result<bool, Error> {
    let permission = permission.into();

    _has_permission(ctx, guild, user, &permission).await
}

async fn _has_permission(
    ctx: &Context,
    guild: GuildId,
    user: UserId,
    permission: &Permission,
) -> Result<bool, Error> {
    permission.allowed(ctx, guild, user).await
}

pub async fn check_permission(
    ctx: &Context,
    guild: GuildId,
    user: UserId,
    permission: impl Into<Permission> + Send,
) -> Result<(), GeneralError> {
    let permission = permission.into();

    if _has_permission(ctx, guild, user, &permission).await? {
        Ok(())
    } else {
        Err(CommandError::PermissionDenied(permission).into())
    }
}

pub struct PermissionHelper {
    ctx: Context,
    guild_id: GuildId,
    user_id: UserId,
}

impl PermissionHelper {
    pub fn new(ctx: Context, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            ctx,
            guild_id,
            user_id,
        }
    }

    pub async fn has_permission(
        &self,
        permission: impl Into<Permission> + Send,
    ) -> Result<bool, Error> {
        has_permission(&self.ctx, self.guild_id, self.user_id, permission).await
    }

    pub async fn check_permission(
        &self,
        permission: impl Into<Permission> + Send,
    ) -> Result<(), GeneralError> {
        check_permission(&self.ctx, self.guild_id, self.user_id, permission).await
    }
}
