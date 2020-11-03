use serenity::Error as SerenityError;
use std::error::Error as StdError;
use std::fmt::{self, Formatter, Display};
use crate::command::Error as CommandError;

#[derive(Debug)]
pub enum Error {
    SerenityError(SerenityError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SerenityError(why) => Display::fmt(why, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::SerenityError(why) => Some(why),
        }
    }
}
