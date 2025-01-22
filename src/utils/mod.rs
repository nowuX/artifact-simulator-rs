mod display;
/// Values for each main/sub Stat
pub mod values;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use values::{main_stat, sub_stats};

use crate::{Artifact, Sets, Stat, Stats, Types};

impl Distribution<Sets> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sets {
        match rng.gen_bool(0.5) {
            true => Sets::Emblem,
            false => Sets::Shimenawa,
        }
    }
}

impl Distribution<Types> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Types {
        match rng.gen_range(0..5) {
            0 => Types::Circlet,
            1 => Types::Flower,
            2 => Types::Goblet,
            3 => Types::Plume,
            _ => Types::Sand,
        }
    }
}

impl Artifact {
    /// Creates a new `Artifact` instance with the specified set, slot, main_stat, sub_stats and upgrades_remaining.
    ///
    /// # Parameters
    /// - `set`: A [`Sets`] enum type.
    /// - `slot`: A [`Types`] enum type.
    /// - `main_stat`: A [Stat] enum type.
    /// - `sub_stats`: Four Optional [`Stat`] values.
    /// - `upgrades_remaining`: A `u8` value [default: 5].
    ///
    /// # Example
    /// ```
    /// use ras::{Artifact, Stats, Sets, Types, Stat};
    ///
    /// let artifact = Artifact::from(
    ///     Sets::Emblem,
    ///     Types::Flower,
    ///     Stat::from(Stats::Hp, 100.0),
    ///     [None, None, None, None],
    ///     5,
    /// );
    /// println!("{artifact:?}");
    /// ```
    pub fn from(
        set: Sets,
        slot: Types,
        main_stat: Stat,
        sub_stats: [Option<Stat>; 4],
        _upgrades_remaining: u8,
    ) -> Self {
        Self {
            set,
            slot,
            main_stat,
            sub_stats,
            _upgrades_remaining,
        }
    }

    /// Return a random `Artifact` instance.
    ///
    /// # Example
    /// ```
    /// use ras::Artifact;
    ///
    /// let artifact = Artifact::random();
    /// println!("{artifact:?}")
    /// ```
    pub fn random() -> Artifact {
        let mut rng = rand::thread_rng();
        let strongbox = rand::thread_rng().gen_bool(0.5);

        let set = Standard.sample(&mut rng);
        let slot = Standard.sample(&mut rng);
        let main = main_stat(&mut rng, &slot);
        let sub = sub_stats(&mut rng, main.stat(), strongbox);

        Artifact::from(set, slot, main, sub, 5)
    }

    /// Return a random `Artifact` instance with a set seed rng.
    ///
    /// # Parameters
    /// - `rng`: a [`Rng`] trait
    /// - `strongbox`: a bool type
    ///
    /// #Example
    /// ```
    /// use ras::Artifact;
    ///
    /// let artifact = Artifact::custom_rng(&mut rand::thread_rng(), false);
    /// println!("{artifact:?}")
    /// ```
    pub fn custom_rng(rng: &mut impl Rng, strongbox: bool) -> Artifact {
        let set = Standard.sample(rng);
        let slot = Standard.sample(rng);

        let main = main_stat(rng, &slot);
        let sub = sub_stats(rng, main.stat(), strongbox);

        Artifact::from(set, slot, main, sub, 5)
    }

    /// Return a `Types` reference
    pub fn slot(&self) -> &Types {
        &self.slot
    }

    /// Return a `Sets` reference
    pub fn set(&self) -> &Sets {
        &self.set
    }

    /// Return a `Stat` reference
    pub fn main_stat(&self) -> &Stat {
        &self.main_stat
    }

    /// Return a reference of the sub_stats
    pub fn sub_stats(&self) -> &[Option<Stat>; 4] {
        &self.sub_stats
    }
}

impl Stat {
    /// Creates a new `Stat` instance with the specified stat type and value.
    ///
    /// # Parameters
    /// - `stat`: A [`Stats`] enum.
    /// - `value`: A floating-point number (`f64`) representing the stat's value.
    ///
    /// # Example
    /// ```
    /// use ras::{Stat, Stats};
    ///
    /// let my_stat = Stat::from(Stats::Atk, 100.0);
    /// println!("{my_stat:?}");
    /// ```
    pub fn from(stat: Stats, value: f64) -> Self {
        Self { stat, value }
    }

    /// Return a reference to the stat type of the current [`Stat`].
    ///
    /// # Example
    /// ```
    /// use ras::{Stat, Stats};
    ///
    /// let my_stat = Stat::from(Stats::Atk, 100.0);
    /// let stat = my_stat.stat();
    /// println!("{stat:?}");
    /// ```
    pub fn stat(&self) -> &Stats {
        &self.stat
    }

    /// Return a reference to the value of the current [`Stat`].
    ///
    /// # Example
    /// ```
    /// use ras::{Stat, Stats};
    ///
    /// let my_stat = Stat::from(Stats::Atk, 100.0);
    /// let value = my_stat.value();
    /// println!("{value:?}");
    /// ```
    pub fn value(&self) -> &f64 {
        &self.value
    }
}
