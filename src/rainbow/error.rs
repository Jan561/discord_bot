use r6stats_client::stats::model::seasonal::Rank;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error {
    UnrecognisedRank(Rank),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::UnrecognisedRank(rank) => write!(f, "Unrecognised Rank: {}", rank),
        }
    }
}

impl StdError for Error {}
