use crate::error::Error;
use r6stats_client::stats::model::seasonal::{Rank, Season};
use r6stats_client::Client as StatsClient;
use r6stats_client::{Platform, Region};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Player {
    pub uplay: Uplay,
}

impl Player {
    pub async fn rank(&self, client: &StatsClient) -> Result<Option<Rank>, Error> {
        let seasonal_stats = client.stats().seasonal(&self.uplay, Platform::Pc).await?;

        let rank = seasonal_stats
            .seasons
            .get(&Season::current_season())
            .and_then(|s| s.regions.get(&Region::Emea))
            .and_then(|r| r.first())
            .map(|r| r.rank);

        Ok(rank)
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Uplay(pub String);

impl PartialEq for Uplay {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl AsRef<str> for Uplay {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Uplay {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
