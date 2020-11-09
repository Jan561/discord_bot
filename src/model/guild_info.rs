use crate::rainbow::model::Player;
use serenity::model::id::UserId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct GuildInfo {
    pub player_map: Arc<RwLock<HashMap<UserId, Player>>>,
}

impl GuildInfo {
    pub fn new() -> Self {
        Self {
            player_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
