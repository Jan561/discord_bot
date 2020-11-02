use crate::rainbow::command::*;
use serenity::framework::standard::macros::group;
use serenity::model::user::User;

#[group]
#[commands(link)]
#[only_in(guilds)]
pub struct Rainbow;

pub struct CommandError {
    notify: NotifyMode,
    kind: Kind,
}

pub enum NotifyMode {
    DontNotify,
    Reflexive,
    About(User),
}

enum Kind {
    RainbowError()
}
