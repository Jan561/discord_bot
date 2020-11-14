use crate::data::GuildInfoMap;
use crate::model::GuildInfo;
use crate::rainbow;
use log::{debug, info};
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use std::sync::Arc;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        info!("Building Cache");

        debug!("Filling Guild Info Map");
        fill_guild_info_map(&ctx, &guilds).await;

        info!("Cache built");

        info!("Starting workers");

        rainbow::worker::rank_updater::spawn_worker(Arc::new(ctx));

        info!("Workers started");
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

async fn fill_guild_info_map(ctx: &Context, guilds: &Vec<GuildId>) {
    let data = ctx.data.read().await;
    let mut guild_info_map = data.get::<GuildInfoMap>().unwrap().write().await;

    for &guild in guilds {
        guild_info_map.insert(guild, GuildInfo::new());
    }
}
