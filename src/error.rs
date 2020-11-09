use crate::command::Error as CommandError;
use serenity::model::id::{GuildId, RoleId};
use serenity::Error as SerenityError;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum GeneralError {
    ExecutionError(Error),
    CommandError(CommandError),
}

impl From<Error> for GeneralError {
    fn from(err: Error) -> Self {
        Self::ExecutionError(err)
    }
}

impl From<CommandError> for GeneralError {
    fn from(err: CommandError) -> Self {
        Self::CommandError(err)
    }
}

#[derive(Debug)]
pub enum Error {
    SerenityError(SerenityError),
    NoGuildInfo(GuildId),
    CacheGuildMissing(GuildId),
    CacheRoleMissing(RoleId),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SerenityError(why) => Display::fmt(why, f),
            Self::NoGuildInfo(guild) => write!(f, "Guild not present in map: {}", guild),
            Self::CacheGuildMissing(guild) => write!(f, "Guild not cached: {}", guild),
            Self::CacheRoleMissing(role) => write!(f, "Role not cached: {}", role),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::SerenityError(why) => Some(why),
            Self::NoGuildInfo(_) | Self::CacheGuildMissing(_) | Self::CacheRoleMissing(_) => None,
        }
    }
}

impl From<SerenityError> for Error {
    fn from(err: SerenityError) -> Self {
        Self::SerenityError(err)
    }
}
