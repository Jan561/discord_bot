use crate::command::Error as CommandError;
use crate::rainbow::model::Uplay;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    UplayTaken(Uplay),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::UplayTaken(_) => write!(f, "This Uplay account is already taken."),
        }
    }
}

impl StdError for Error {}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        Self::RainbowError(err)
    }
}
