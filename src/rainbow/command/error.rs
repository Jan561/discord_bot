use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl StdError for Error {}
