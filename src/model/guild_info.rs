use crate::rainbow::model::Player;
use serenity::model::id::UserId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct GuildInfo {
    player_map: Arc<RwLock<HashMap<UserId, Player>>>,
}
