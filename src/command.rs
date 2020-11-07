use crate::rainbow::command::Error as RainbowError;
use crate::rainbow::command::*;
use serenity::framework::standard::macros::group;
use serenity::model::user::User;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[group]
#[commands(link)]
#[only_in(guilds)]
pub struct Rainbow;

#[derive(Debug)]
pub enum Error {
    RainbowError(RainbowError),
    ArgumentMissing(Option<String>),
    PermissionDenied,
    UserNotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::RainbowError(why) => Display::fmt(why, f),
            Self::ArgumentMissing(arg) => {
                let arg = match arg {
                    Some(arg) => format!(": {}", arg),
                    None => "".to_string(),
                };

                write!(f, "Argument missing{}.", arg)
            }
            Self::PermissionDenied => write!(f, "Permission is denied."),
            Self::UserNotFound(user) => write!(f, "User not found: {}.", user),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::RainbowError(err) => Some(err),
            Self::ArgumentMissing(_) | Self::PermissionDenied => None,
        }
    }
}
