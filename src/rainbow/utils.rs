use super::model::Player;
use crate::data::GuildInfoMap;
use crate::Error;
use serenity::client::Context;
use serenity::model::id::{GuildId, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn player_map(
    ctx: &Context,
    guild_id: GuildId,
) -> Result<Arc<RwLock<HashMap<UserId, Player>>>, Error> {
    let data = ctx.data.read().await;
    let guild_info_map = data.get::<GuildInfoMap>().unwrap().read().await;
    let player_map = &guild_info_map
        .get(&guild_id)
        .ok_or(Error::NoGuildInfo(guild_id))?
        .player_map;

    Ok(Arc::clone(player_map))
}
