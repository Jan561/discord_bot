use crate::rainbow::command::Error as RainbowError;
use crate::rainbow::command::*;
use crate::Permission;
use serenity::framework::standard::macros::group;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[group]
#[commands(link)]
#[only_in(guilds)]
pub struct Rainbow;

#[derive(Debug)]
pub enum Error {
    RainbowError(RainbowError),
    ArgumentMissing(String),
    PermissionDenied(Permission),
    UserNotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::RainbowError(why) => Display::fmt(why, f),
            Self::ArgumentMissing(arg) => write!(f, "Argument missing: {}.", arg),
            Self::PermissionDenied(_) => write!(f, "Permission is denied."),
            Self::UserNotFound(user) => write!(f, "User not found: {}.", user),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::RainbowError(err) => Some(err),
            Self::ArgumentMissing(_) | Self::PermissionDenied(_) | Self::UserNotFound(_) => None,
        }
    }
}
