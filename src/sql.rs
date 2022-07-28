pub struct DiscordUser {
    pub id: Option<Id>,
    pub discord_id: Option<DiscordId>,
}

pub type Id = u64;
pub type DiscordId = u64;