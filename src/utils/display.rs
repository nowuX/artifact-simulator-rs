use std::fmt::Display;

use super::{Artifact, Sets, Stat, Stats, Types};

impl Display for Artifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sub_stats = self
            .sub_stats()
            .iter()
            .filter(|item| item.is_some())
            .map(|item| {
                if let Some(value) = item {
                    format!("{value}")
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n\t");
        write!(
            f,
            "{} - {}\n{}\n\t{}",
            self.slot(),
            self.set(),
            self.main_stat(),
            sub_stats
        )
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:.2}", self.stat(), self.value())
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            }
        )
    }
}
