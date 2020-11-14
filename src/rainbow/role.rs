use crate::error::Error as CrateError;
use crate::rainbow::Error;
use r6stats_client::stats::model::seasonal::Rank;

pub fn rank_to_role(rank: Rank) -> Result<&'static str, CrateError> {
    if rank.is_unranked() {
        Ok("Unranked")
    } else if rank.is_copper() {
        Ok("Copper")
    } else if rank.is_bronze() {
        Ok("Bronze")
    } else if rank.is_silver() {
        Ok("Silver")
    } else if rank.is_gold() {
        Ok("Gold")
    } else if rank.is_platinum() {
        Ok("Platinum")
    } else if rank.is_diamond() {
        Ok("Diamond")
    } else if rank.is_champion() {
        Ok("Champion")
    } else {
        Err(Error::UnrecognisedRank(rank).into())
    }
}
