use crate::data::StatsClientContainer;
use crate::error::Error;
use crate::rainbow::role::rank_to_role;
use futures::StreamExt;
use log::{error, info, warn};
use r6stats_client::stats::model::seasonal::Rank;
use serenity::client::Context;
use serenity::model::id::UserId;
use serenity::static_assertions::_core::time::Duration;
use std::sync::Arc;
use tokio::time::delay_for;

const UPDATE_INTERVAL: u64 = 300;

pub fn spawn_worker(ctx: Arc<Context>) {
    tokio::spawn(async move {
        let stats_client = {
            let data = ctx.data.read().await;
            Arc::clone(data.get::<StatsClientContainer>().unwrap())
        };

        loop {
            info!("Updating ranks");

            let guild_ids = ctx.cache.guilds().await;

            for guild_id in guild_ids {
                let player_map_lock = match player_map!(ctx, guild_id) {
                    Ok(lock) => lock,
                    Err(why) => {
                        error!("Error getting GuildInfo: {:?}", why);
                        continue;
                    }
                };

                let player_map = player_map_lock.read().await;

                // Make lifetime of `stats_client` static so there are no problems with async blocks
                // Note(Safety): This is safe as `stats_client` never gets deallocated (infinite loop)
                #[allow(unused_unsafe)]
                let stats_client1 = unsafe { &*stats_client };

                let x = futures::stream::iter(player_map.iter())
                    .then(|(&user_id, player)| async move {
                        (user_id, player.rank(&stats_client1).await)
                    })
                    .collect::<Vec<(UserId, Result<Option<Rank>, Error>)>>()
                    .await;

                // Note(Safety): This is safe as `ctx` never gets deallocated (infinite loop)
                #[allow(unused_unsafe)]
                let ctx1 = unsafe { &*ctx };

                futures::stream::iter(x.into_iter())
                    .for_each(|(user_id, rank)| async move {
                        let mut member = match guild_id.member(&ctx1.http, user_id).await {
                            Ok(member) => member,
                            Err(why) => {
                                error!("Error getting member: {}", why);
                                return;
                            }
                        };

                        let rank = match rank {
                            Ok(Some(rank)) => rank,
                            Ok(None) => {
                                warn!("Couldn't update rank for {}", member.user.tag());
                                return;
                            }
                            Err(why) => {
                                error!("Error getting rank: {}", why);
                                return;
                            }
                        };

                        let role_str = match rank_to_role(rank) {
                            Ok(role) => role,
                            Err(why) => {
                                error!("Error converting rank to role: {}", why);
                                return;
                            }
                        };

                        let guild = match guild_id
                            .to_guild_cached(&ctx1.cache)
                            .await
                            .ok_or(Error::CacheGuildMissing(guild_id))
                        {
                            Ok(guild) => guild,
                            Err(why) => {
                                error!("{}", why);
                                return;
                            }
                        };

                        let role = match guild
                            .role_by_name(role_str)
                            .cloned()
                            .ok_or_else(|| Error::UnrecognisedRole(role_str.to_string()))
                        {
                            Ok(role) => role,
                            Err(why) => {
                                error!("{}", why);
                                return;
                            }
                        };

                        if let Err(why) = member.add_role(&ctx1.http, role).await {
                            error!("Error assigning role to member: {}", why);
                            return;
                        }
                    })
                    .await;
            }

            info!("Finished updating ranks");

            delay_for(Duration::from_secs(UPDATE_INTERVAL)).await;
        }
    });
}
