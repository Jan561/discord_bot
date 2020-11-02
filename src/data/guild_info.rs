use crate::model::GuildInfo;
use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct GuildInfoMap;

impl TypeMapKey for GuildInfoMap {
    type Value = Arc<RwLock<HashMap<GuildId, GuildInfo>>>;
}
