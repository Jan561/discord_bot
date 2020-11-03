use crate::rainbow::command::*;
use serenity::framework::standard::macros::group;
use serenity::model::user::User;
use std::error::Error as StdError;
use std::fmt::{self, Formatter, Display};

#[group]
#[commands(link)]
#[only_in(guilds)]
pub struct Rainbow;

#[derive(Debug)]
pub enum Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            _ => unimplemented!(),
        }
    }
}

impl StdError for Error {}
