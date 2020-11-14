use r6stats_client::Client;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub struct StatsClientContainer;

impl TypeMapKey for StatsClientContainer {
    type Value = Arc<Client>;
}
