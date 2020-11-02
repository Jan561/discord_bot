use serenity::model::id::{GuildId, UserId};

#[derive(Clone, Debug)]
pub struct Player {
    user: UserId,
    guild: GuildId,
    player: String,
}
