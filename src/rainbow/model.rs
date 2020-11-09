use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Player {
    pub uplay: Uplay,
}

#[derive(Clone, Debug, Eq)]
pub struct Uplay(pub String);

impl PartialEq for Uplay {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Display for Uplay {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
