use crate::command::Error as CommandError;
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
    pub async fn allowed(self, ctx: &Context, guild: GuildId, user: UserId) -> Result<bool, Error> {
        match self {
            Self::RainbowPermission(p) => p.allowed(ctx, guild, user).await,
        }
    }
}

pub async fn owner(ctx: &Context, guild: GuildId) -> Result<UserId, Error> {
    Ok(guild
        .to_guild_cached(&ctx.cache)
        .await
        .ok_or(Error::CacheGuildMissing(guild))?
        .owner_id)
}

pub async fn is_admin(ctx: &Context, guild: GuildId, user: UserId) -> Result<bool, Error> {
    if user == owner(ctx, guild).await? {
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

pub async fn check_permission(
    ctx: &Context,
    guild: GuildId,
    user: UserId,
    permission: Permission,
) -> Result<Result<(), CommandError>, Error> {
    if user.has_permission(ctx, guild, permission).await? {
        Ok(Ok(()))
    } else {
        Ok(Err(CommandError::PermissionDenied))
    }
}

pub struct PermissionFacility {
    ctx: Context,
    guild_id: GuildId,
    user_id: UserId,
}

impl PermissionFacility {
    pub fn new(ctx: Context, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            ctx,
            guild_id,
            user_id,
        }
    }

    pub async fn has_permission(&self, permission: Permission) -> Result<bool, Error> {
        self.user_id
            .has_permission(&self.ctx, self.guild_id, permission)
            .await
    }

    pub async fn check_permission(
        &self,
        permission: Permission,
    ) -> Result<Result<(), CommandError>, Error> {
        check_permission(&self.ctx, self.guild_id, self.user_id, permission).await
    }
}
