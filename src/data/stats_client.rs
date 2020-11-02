use r6stats_client::Client;
use serenity::prelude::TypeMapKey;

pub struct StatsClientContainer;

impl TypeMapKey for StatsClientContainer {
    type Value = Client;
}
