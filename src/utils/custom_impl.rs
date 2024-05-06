use crate::utils::WeightedStat;
use crate::{Artifact, Sets, Stat, Stats, SubStats, Types};

impl Artifact {
    /// Generate a new [Artifact] with a fixed [`set`](Sets), [`type`](Types), [`main stat`](Stat) and
    /// [`sub stats`](SubStats).
    ///
    /// #### Example
    /// ```
    /// use ags::{Artifact, Sets, Stat, Stats, SubStats, Types};
    ///
    /// let main_stat = Stat::new(Stats::CR, 64.5);
    /// let sub_stats = SubStats::default();
    /// let artifact = Artifact::new(Sets::Emblem, Types::Circlet, main_stat, sub_stats, 5);
    /// ```
    pub fn new(
        set: Sets,
        slot: Types,
        main_stat: Stat,
        sub_stats: SubStats,
        upgrades_remaining: i32,
    ) -> Self {
        Self {
            set,
            slot,
            main_stat,
            sub_stats,
            upgrades_remaining,
        }
    }
}

impl WeightedStat {
    /// Generate a new WeightedStat with specific stat and weight
    ///
    /// #### Example
    /// ```
    ///
    /// use ags::Stats;
    /// use ags::utils::WeightedStat;
    ///
    /// let stats_with_weight = vec![WeightedStat::new(Stats::Hp, 20)];
    /// ```
    pub fn new(stat: Stats, weight: i32) -> Self {
        Self { stat, weight }
    }
}

impl Stat {
    /// Generate a new [`Stat`] with stat and a f64 value fixed
    ///
    /// #### Example
    /// ```
    /// use ags::{Stat, Stats};
    ///
    /// let stat = Stat::new(Stats::CD, 9.8);
    /// ```
    pub fn new(stat: Stats, value: f64) -> Self {
        Self { stat, value }
    }

    /// Generate a new [`Stat`] with stat and an i32 value fixed
    ///
    /// #### Example
    /// ```
    /// use ags::{Stat, Stats};
    ///
    /// let stat = Stat::new_i32(Stats::Hp, 123);
    /// ```
    pub fn new_i32(stat: Stats, i32_value: i32) -> Self {
        let value = i32_value as f64;
        Self { stat, value }
    }
}

impl SubStats {
    /// Add a new stat in an empty [`SubStats`] slot.
    ///
    /// #### Example
    /// ```
    /// use ags::{Stat, Stats, SubStats};
    ///
    /// let mut sub_stats = SubStats::default();
    /// assert!(sub_stats.first.is_none());
    /// sub_stats.add_stat(Stat::new(Stats::CR, 9.8));
    /// assert!(sub_stats.first.is_some());
    /// ```
    pub fn add_stat(&mut self, new_stat: Stat) {
        match self.as_array_mut().iter_mut().find(|x| x.is_none()) {
            None => panic!("no empty stat field to add stat"),
            Some(empty_stat) => **empty_stat = Some(new_stat),
        }
    }

    pub fn as_array_mut(&mut self) -> [&mut Option<Stat>; 4] {
        [
            &mut self.first,
            &mut self.second,
            &mut self.third,
            &mut self.fourth,
        ]
    }

    pub fn as_array(&self) -> [Option<&Stat>; 4] {
        [
            self.first.as_ref(),
            self.second.as_ref(),
            self.third.as_ref(),
            self.fourth.as_ref(),
        ]
    }
}
