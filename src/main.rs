mod command;
mod data;
mod handler;
mod hook;
mod model;
mod rainbow;
mod error;

use crate::command::*;
use crate::data::{GuildInfoMap, StatsClientContainer};
use crate::handler::Handler;
use crate::hook::{after, before};
use r6stats_client::Client as StatsClient;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::id::UserId;
use serenity::Client;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

const ENV_DISCORD: &str = "DISCORD_TOKEN";
const ENV_R6STATS: &str = "R6STATS_TOKEN";

#[tokio::main]
async fn main() {
    let http = http();

    let owners = owners(&http).await;
    let bot_id = bot_id(&http).await;

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!").on_mention(Some(bot_id)))
        .before(before)
        .after(after)
        .group(&RAINBOW_GROUP);

    let mut client = Client::builder(discord_token())
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<GuildInfoMap>(Arc::new(RwLock::new(HashMap::new())));
        data.insert::<StatsClientContainer>(stats_client());
    }

    client.start().await.expect("Error starting client");
}

fn discord_token() -> String {
    env::var(ENV_DISCORD).expect("Discord token not in env")
}

fn r6stats_token() -> String {
    env::var(ENV_R6STATS).expect("r6stats token not in env")
}

fn http() -> Http {
    Http::new_with_token(&discord_token())
}

async fn owners(http: &Http) -> HashSet<UserId> {
    let app_info = http
        .get_current_application_info()
        .await
        .expect("Couldn't access application info");

    let mut set = HashSet::new();
    set.insert(app_info.owner.id);
    set
}

async fn bot_id(http: &Http) -> UserId {
    http.get_current_user()
        .await
        .expect("Couldn't get bot id")
        .id
}

fn stats_client() -> StatsClient {
    StatsClient::new(r6stats_token()).expect("Error creating stats client")
}
