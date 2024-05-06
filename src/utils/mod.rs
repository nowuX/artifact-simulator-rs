use crate::Stats;

pub mod artifact;
mod custom_impl;
mod display_impl;

/// Struct helper for [`WeightedIndex`](rand::distributions::WeightedIndex)
pub struct WeightedStat {
    /// Choice stat
    pub stat: Stats,
    /// Corresponding weight of the stat
    pub weight: i32,
}
