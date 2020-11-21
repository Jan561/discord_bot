macro_rules! say {
    ($ctx:expr, $channel_id:expr, $($content:expr),*) => {{
        $channel_id.say(&$ctx.http, format!($($content),*)).await
    }};
}

macro_rules! member {
    ($ctx:expr, $msg:expr, $user:expr) => {{
        $msg.guild(&$ctx.cache)
            .await
            .ok_or(crate::Error::CacheGuildMissing($msg.guild_id.unwrap()))?
            .member_named($user)
            .ok_or(crate::command::Error::UserNotFound($user.to_string()))?
            .clone()
    }};
}

macro_rules! player_map {
    ($ctx:expr, $guild_id:expr) => {{
        let guild_info_map_lock = {
            let data = $ctx.data.read().await;
            std::sync::Arc::clone(data.get::<crate::data::GuildInfoMap>().unwrap())
        };
        let guild_info_map = guild_info_map_lock.read().await;
        guild_info_map
            .get(&$guild_id)
            .ok_or(crate::Error::NoGuildInfo($guild_id))
            .map(|guild_info| std::sync::Arc::clone(&guild_info.player_map))
    }};
}

/*macro_rules! db {
    (
        $vis:vis struct $ident:ident $(<$($generic:ident $(: $bound:ty)?),* $(,)?>)?
        $(
            where
            $($where_gen:ident : $where_bound:ty),+ $(,)?
        )?
        {
            $($field:tt)*
        }
    ) => {
        $vis struct $ident<__State, $($generic),*>
        $(
            where
            $($where_gen : $where_bound),*
        )?
        {

        }
    };
}
*/