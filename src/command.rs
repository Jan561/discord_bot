use crate::error::GeneralError;
use crate::rainbow::command::Error as RainbowError;
use crate::rainbow::command::*;
use crate::Error as ExecutionError;
use crate::Permission;
use log::warn;
use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::future::Future;

#[group]
#[commands(link, unlink)]
#[only_in(guilds)]
pub struct Rainbow;

#[derive(Debug)]
pub enum Error {
    RainbowError(RainbowError),
    ArgumentMissing(String),
    PermissionDenied(Permission),
    UserNotFound(String),
}

impl Error {
    pub async fn notify_author(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
    ) -> Result<(), ExecutionError> {
        say!(ctx, channel_id, "{}", self)?;

        Ok(())
    }
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

pub async fn execute<F>(ctx: &Context, msg: &Message, f: F) -> CommandResult
where
    F: Future<Output = Result<(), GeneralError>>,
{
    match f.await {
        Ok(()) => Ok(()),
        Err(crate::error::GeneralError::ExecutionError(why)) => Err(Box::new(why)),
        Err(crate::error::GeneralError::CommandError(why)) => {
            why.notify_author(ctx, msg.channel_id).await?;

            warn!("Command Error: {}", why);

            Ok(())
        }
    }
}
