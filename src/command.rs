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
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::RainbowError(why) => Display::fmt(why, f),
            Self::ArgumentMissing(arg) => {
                let arg = match arg {
                    Some(arg) => ": " + arg,
                    None => "".to_string(),
                };

                write!(f, "Argument missing{}.", arg)
            }
        }
    }
}

impl StdError for Error {}
