macro_rules! say {
    ($ctx:expr, $channel_id:expr, $($content:expr),*) => {
        $channel_id.say(&$ctx.http, format!($($content),*)).await
    }
}
