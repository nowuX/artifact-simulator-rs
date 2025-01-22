//! Rust Artifact Generator Simulator

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Logic of the artifact generator
pub mod utils;

#[derive(Debug)]
/// Artifact Sets are different kinds of Artifacts that have different Set Bonuses depending on how
/// many are of that set are equipped on a Character.
/// [`Source`](<https://genshin-impact.fandom.com/wiki/Artifact/Sets>)
pub enum Sets {
    /// 2 Piece: [`Energy Recharge`](Stats::ER) `+20%`
    ///
    /// 4 Piece: Increases Elemental Burst DMG by `25%` of
    /// [`Energy Recharge`](Stats::ER). A maximum of 75% bonus DMG can be obtained in this way.
    /// [`Source`](<https://genshin-impact.fandom.com/wiki/Emblem_of_Severed_Fate>).
    Emblem,
    /// 2 Piece: [`ATK`](Stats::AtkPct) `+18%`
    ///
    /// 4 Piece: When casting an Elemental Skill, if the [`Character`](crate::structs::Character)
    /// has 15 or more Energy, they lose 15 Energy and Normal/Charged/Plunging
    /// [`Attack DMG`](Stats::Atk) is increased by `50%` for 10s. This effect will not trigger again
    /// during that duration
    /// [`Source`](<https://genshin-impact.fandom.com/wiki/Shimenawa%27s_Reminiscence>).
    Shimenawa,
}

#[derive(Debug)]
/// There are 5 types of Artifacts that can be equipped. Only one of each type can be equipped on a
/// character at a time.
pub enum Types {
    /// It increases a character's HP stat by adding flat HP and is one of the two Artifact Pieces
    /// to have a set main stat (along with the Plume of Death).
    Flower,
    /// It increases a character's ATK stat by adding flat ATK and is one of the two Artifact
    /// Pieces to have a set main stat (along with the Flower of Life).
    Plume,
    /// It increases a character's stats and has 5 possible main stats: HP%, DEF%, ATK%, Elemental
    /// Mastery, or Energy Recharge. Artifact Pieces can appear in all rarities (1–5 stars).
    Sand,
    /// It increases a character's stats and currently has 12 possible main stats:
    /// HP%, DEF%, ATK%, Elemental Mastery, or
    /// Physical/Hydro/Pyro/Cryo/Dendro/Electro/Anemo/Geo DMG Bonus%.
    Goblet,
    /// It can increase a character's stats and currently has 7 possible main stats:
    /// HP%, DEF%, ATK%, Elemental Mastery, CRIT Rate%, CRIT DMG%, or Healing Bonus%.
    Circlet,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
/// Attributes (more commonly known as Stats)
/// [`Source`](<https://genshin-impact.fandom.com/wiki/Artifact/Stats>)
pub enum Stats {
    Atk,
    Hp,
    Def,
    AtkPct,
    HpPct,
    DefPct,
    EM,
    ER,
    ElementalDMG,
    PhysicalDMG,
    OtherDMG,
    CR,
    CD,
    HB,
}

#[derive(Debug)]
/// Each [Artifact] has a `Main Stat` and up to 4 [`Sub Stats`](SubStats).
pub struct Stat {
    /// For the `Main Stat` this stats can be specific as for [`Flower`](Types::Flower) and
    /// [`Plume`](Types::Plume) or it can have more variety like the others artifacts types.
    ///
    /// For the [`Sub Stats`](SubStats) an artifact cannot have any of them repeated and cannot have
    /// the same stat as the `Main Stat`.
    /// [`Source`](<https://genshin-impact.fandom.com/wiki/Artifact/Stats#Sub_Stats>).
    stat: Stats,
    /// For the `Main Stat` this is fixed value and always dependent on the rarity of the
    /// [Artifact].
    ///
    /// For the [`Sub Stats`](SubStats) this value is random in the range of 100% - 70% and
    /// dependent on the rarity of the [Artifact].
    value: f64,
}

#[derive(Debug)]
/// Item in `Genshin Impact` that can be equipped on Characters to increase their Stats.
/// [`Source`](<https://genshin-impact.fandom.com/wiki/Artifact>)
pub struct Artifact {
    /// Of the two 5-star [Sets] that can come out in a domain both have the same probability of
    /// being generated.
    /// [Source](<https://genshin-impact.fandom.com/wiki/Loot_System/Artifact_Drop_Distribution#Domains_3>)
    set: Sets,
    /// Of the five [Types] that could be generated, all have the same probability.
    /// [Source](<https://genshin-impact.fandom.com/wiki/Loot_System/Artifact_Drop_Distribution#Artifact_Type_Distribution>)
    slot: Types,
    /// Each artifact has one `main stat`, determined by the artifact's type/slot.
    /// Its starting value is determined by the artifact's rarity.
    main_stat: Stat,
    /// Along with the main stat, artifacts can have up to 4 sub-stats; the starting number is based on
    /// its rarity.
    ///
    /// An artifact cannot have duplicate sub-stats, and its sub-stats cannot be the same as the
    /// main stat. (E.g., an artifact with an ATK% main stat cannot gain an ATK% sub-stat, but could
    /// have a flat ATK sub-stat.)
    sub_stats: [Option<Stat>; 4],
    /// Each artifact has just 5 times that can be upgraded
    _upgrades_remaining: u8,
}

#[cfg(test)]
mod tests {

    use rand::{rngs::StdRng, SeedableRng};

    use crate::Artifact;

    #[test]
    fn test_artifact() {
        let mut rng = StdRng::seed_from_u64(24);
        let artifact = Artifact::custom_rng(&mut rng, false);
        println!("{artifact}");
    }
}
