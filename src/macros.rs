macro_rules! say {
    ($ctx:expr, $channel_id:expr, $($content:expr),*) => {
        $channel_id.say(&$ctx.http, format!($($content),*)).await
    }
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
