use rusqlite::Connection;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Database;

impl TypeMapKey for Database {
    type Value = Arc<Mutex<Connection>>;
}
