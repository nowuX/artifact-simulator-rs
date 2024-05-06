//! Rust Artifact Generator Simulator

use rand::Rng;

use crate::utils::artifact::{main_stat, sub_stats};

pub mod utils;

/// Generate a random [Artifact] from a domain.
///
/// #### Example (no strongbox)
/// ```
/// let artifact = ags::generate_artifact(false);
/// ```
pub fn generate_artifact(strongbox: bool) -> Artifact {
    let set = match rand::thread_rng().gen_range(1..=2) {
        1 => Sets::Emblem,
        _ => Sets::Shimenawa,
    };
    let slot = match rand::thread_rng().gen_range(1..=5) {
        1 => Types::Flower,
        2 => Types::Plume,
        3 => Types::Sand,
        4 => Types::Goblet,
        _ => Types::Circlet,
    };
    let main = main_stat(&slot);
    let sub = sub_stats(&main.stat, strongbox);

    Artifact::new(set, slot, main, sub, 5)
}

/// Item in `Genshin Impact` that can be equipped on Characters to increase their Stats.
/// [`Source`](<https://genshin-impact.fandom.com/wiki/Artifact>)
pub struct Artifact {
    /// Of the two 5-star [Sets] that can come out in a domain both have the same probability of
    /// being generated.
    /// [Source](<https://genshin-impact.fandom.com/wiki/Loot_System/Artifact_Drop_Distribution#Domains_3>)
    pub set: Sets,
    /// Of the five [Types] that could be generated, all have the same probability.
    /// [Source](<https://genshin-impact.fandom.com/wiki/Loot_System/Artifact_Drop_Distribution#Artifact_Type_Distribution>)
    pub slot: Types,
    /// Each artifact has one `main stat`, determined by the artifact's type/slot.
    /// Its starting value is determined by the artifact's rarity.
    pub main_stat: Stat,
    /// Along with the main stat, artifacts can have up to 4 sub-stats; the starting number is based on
    /// its rarity.
    ///
    /// An artifact cannot have duplicate sub-stats, and its sub-stats cannot be the same as the
    /// main stat. (E.g., an artifact with an ATK% main stat cannot gain an ATK% sub-stat, but could
    /// have a flat ATK sub-stat.)
    pub sub_stats: SubStats,
    /// Each artifact has just 5 times that can be upgraded
    pub upgrades_remaining: i32,
}

/// Each [Artifact] has a `Main Stat` and up to 4 [`Sub Stats`](SubStats).
#[derive(PartialEq)]
pub struct Stat {
    /// For the `Main Stat` this stats can be specific as for [`Flower`](Types::Flower) and
    /// [`Plume`](Types::Plume) or it can have more variety like the others artifacts types.
    ///
    /// For the [`Sub Stats`](SubStats) an artifact cannot have any of them repeated and cannot have
    /// the same stat as the `Main Stat`.
    /// [`Source`](<https://genshin-impact.fandom.com/wiki/Artifact/Stats#Sub_Stats>).
    pub stat: Stats,
    /// For the `Main Stat` this is fixed value and always dependent on the rarity of the
    /// [Artifact].
    ///
    /// For the [`Sub Stats`](SubStats) this value is random in the range of 100% - 70% and
    /// dependent on the rarity of the [Artifact].
    pub value: f64,
}

#[derive(Default)]
/// SubStats of the [Artifact]
///
/// There is no guarantee that a 5* artefact will have all four sub-stats, so they are all
/// [`Options`](Stat).
pub struct SubStats {
    pub first: Option<Stat>,
    pub second: Option<Stat>,
    pub third: Option<Stat>,
    // TODO make just this stat a option (strict 5* artifact generated)
    pub fourth: Option<Stat>,
}

#[derive(PartialEq, Default, Copy, Clone)]
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
    #[default]
    None,
}

#[derive(PartialEq, Copy, Clone)]
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

#[derive(PartialEq, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_SETS: [Sets; 2] = [Sets::Emblem, Sets::Shimenawa];
    const VALID_TYPES: [Types; 5] = [
        Types::Flower,
        Types::Plume,
        Types::Sand,
        Types::Goblet,
        Types::Circlet,
    ];
    const FLOWER_STATS: [Stats; 1] = [Stats::Hp];
    const PLUME_STATS: [Stats; 1] = [Stats::Atk];
    const SAND_STATS: [Stats; 5] = [
        Stats::HpPct,
        Stats::AtkPct,
        Stats::DefPct,
        Stats::EM,
        Stats::ER,
    ];
    const GOBLET_STATS: [Stats; 7] = [
        Stats::HpPct,
        Stats::AtkPct,
        Stats::DefPct,
        Stats::ElementalDMG,
        Stats::PhysicalDMG,
        Stats::OtherDMG,
        Stats::EM,
    ];
    const CIRCLET_STATS: [Stats; 7] = [
        Stats::HpPct,
        Stats::AtkPct,
        Stats::DefPct,
        Stats::CD,
        Stats::CR,
        Stats::HB,
        Stats::EM,
    ];
    const ITERATIONS: i32 = 10_000;

    #[test]
    fn test_artifacts_fields() {
        use rand::Rng;

        for _ in 0..ITERATIONS {
            let strongbox = rand::thread_rng().gen_bool(0.5);
            let mut sets_mut = VALID_SETS.to_vec();
            let mut types_mut = VALID_TYPES.to_vec();

            let mut flower_mut = FLOWER_STATS.to_vec();
            let mut plume_mut = PLUME_STATS.to_vec();
            let mut sand_mut = SAND_STATS.to_vec();
            let mut goblet_mut = GOBLET_STATS.to_vec();
            let mut circlet_mut = CIRCLET_STATS.to_vec();

            loop {
                let artifact = generate_artifact(strongbox);

                let main = &artifact.main_stat.stat;
                let slot = &artifact.slot;
                let set = &artifact.set;

                assert!(VALID_SETS.contains(set), "Invalid set {}", set);
                sets_mut.retain(|artifact_set| artifact_set == set);

                assert!(VALID_TYPES.contains(slot), "Invalid type {}", slot);
                types_mut.retain(|artifact_slot| artifact_slot == slot);

                let (main_stats, mut_main) = match slot {
                    Types::Flower => (FLOWER_STATS.to_vec(), &mut flower_mut),
                    Types::Plume => (PLUME_STATS.to_vec(), &mut plume_mut),
                    Types::Sand => (SAND_STATS.to_vec(), &mut sand_mut),
                    Types::Goblet => (GOBLET_STATS.to_vec(), &mut goblet_mut),
                    Types::Circlet => (CIRCLET_STATS.to_vec(), &mut circlet_mut),
                };
                assert!(
                    main_stats.contains(main),
                    "Invalid main stat: {} for this artifact type: {}",
                    main,
                    &slot
                );
                mut_main.retain(|main_stat| main_stat == main);

                if let Some(sub_stat) = artifact
                    .sub_stats
                    .as_array()
                    .iter()
                    .flatten()
                    .find(|sub_stat| &sub_stat.stat == main)
                {
                    panic!(
                        "Found:\n\t{}\nHas the same stat of the main_stat:\n\t{}",
                        sub_stat, artifact.main_stat
                    )
                }

                if sets_mut.is_empty() && types_mut.is_empty() && mut_main.is_empty() {
                    break;
                }
            }
        }
    }
}
