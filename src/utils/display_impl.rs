use crate::{Artifact, Sets, Stat, Stats, Types};
use std::fmt::{Display, Formatter};

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Stats::Hp => "HP",
                Stats::HpPct => "HP %",
                Stats::Atk => "ATK",
                Stats::AtkPct => "ATK %",
                Stats::Def => "DEF",
                Stats::DefPct => "DEF %",
                Stats::ER => "Energy Recharge %",
                Stats::EM => "Elemental Mastery",
                Stats::ElementalDMG => "Elemental DMG Bonus %",
                Stats::PhysicalDMG => "Physical DMG Bonus %",
                Stats::OtherDMG => "Other DMG Bonus %",
                Stats::CR => "Critical Rate %",
                Stats::CD => "Critical Damage %",
                Stats::HB => "Healing Bonus %",
                Stats::None => "[None]",
            }
        )
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Types::Flower => "Flower of Life",
                Types::Plume => "Plume of Death",
                Types::Sand => "Sands of Eon",
                Types::Goblet => "Goblet of Eonothem",
                Types::Circlet => "Circlet of Logos",
            }
        )
    }
}

impl Display for Sets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sets::Emblem => "Emblem of Severed Fate",
                Sets::Shimenawa => "Shimenawa's Reminiscence",
            }
        )
    }
}

impl Display for Artifact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}\n{}\n\t{}",
            self.slot,
            self.set,
            self.main_stat,
            self.sub_stats
                .as_array()
                .iter()
                .flatten()
                .map(|sub_stat| format!("{}", sub_stat))
                .collect::<Vec<String>>()
                .join("\n\t")
        )
    }
}

impl Display for Stat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  {:.2}", self.stat, self.value)
    }
}
