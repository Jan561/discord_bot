use crate::data::GuildInfoMap;
use crate::model::GuildInfo;
use log::info;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let data = ctx.data.read().await;
        let mut guild_info_map = data.get::<GuildInfoMap>().unwrap().write().await;

        for guild in guilds {
            guild_info_map.insert(guild, GuildInfo::new());
        }

        info!("Cache built")
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
