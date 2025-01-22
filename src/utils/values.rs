use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    Rng,
};

use super::{Stat, Stats, Types};

/// Return a random [`Stat`] for the artifact main stat
pub fn main_stat(rng: &mut impl Rng, slot: &Types) -> Stat {
    let stat = get_main_stat(rng, slot);
    let value = get_main_value(slot, &stat);

    Stat::from(stat, value)
}

struct WeightedStat(Stats, u16);

/// Returns a random [`Stat`][Stats] for the specific artifact [`Type`](Types)
fn get_main_stat(rng: &mut impl Rng, slot: &Types) -> Stats {
    let stats_weights = match slot {
        Types::Flower => vec![WeightedStat(Stats::Hp, 1)],
        Types::Plume => vec![WeightedStat(Stats::Atk, 1)],
        Types::Sand => vec![
            WeightedStat(Stats::HpPct, 8),
            WeightedStat(Stats::AtkPct, 8),
            WeightedStat(Stats::DefPct, 8),
            WeightedStat(Stats::EM, 3),
            WeightedStat(Stats::ER, 3),
        ],
        Types::Goblet => vec![
            WeightedStat(Stats::HpPct, 77),
            WeightedStat(Stats::AtkPct, 77),
            WeightedStat(Stats::DefPct, 76),
            WeightedStat(Stats::ElementalDMG, 20),
            WeightedStat(Stats::PhysicalDMG, 20),
            WeightedStat(Stats::OtherDMG, 120),
            WeightedStat(Stats::EM, 10),
        ],
        Types::Circlet => vec![
            WeightedStat(Stats::HpPct, 11),
            WeightedStat(Stats::AtkPct, 11),
            WeightedStat(Stats::DefPct, 11),
            WeightedStat(Stats::CD, 5),
            WeightedStat(Stats::CR, 5),
            WeightedStat(Stats::HB, 5),
            WeightedStat(Stats::EM, 2),
        ],
    };

    if stats_weights.len() == 1 {
        return stats_weights.first().unwrap().0.clone();
    }

    let weights = stats_weights.iter().map(|w| w.1);
    let dist = match WeightedIndex::new(weights) {
        Ok(r) => r,
        Err(_) => panic!("ERROR: failed at mapping weights values"),
    };
    let index = dist.sample(rng);

    match stats_weights.get(index) {
        Some(stat) => stat.0.clone(),
        None => panic!("ERROR: failed indexing stat for {slot:?} stats pool"),
    }
}

fn get_main_value(slot: &Types, main_stat: &Stats) -> f64 {
    let stats_values = match slot {
        Types::Flower => vec![Stat::from(Stats::Hp, 4780.0)],
        Types::Plume => vec![Stat::from(Stats::Atk, 311.0)],
        Types::Sand => vec![
            Stat::from(Stats::HpPct, 46.6),
            Stat::from(Stats::AtkPct, 46.6),
            Stat::from(Stats::DefPct, 58.3),
            Stat::from(Stats::ER, 51.8),
            Stat::from(Stats::EM, 186.5),
        ],
        Types::Goblet => vec![
            Stat::from(Stats::HpPct, 46.6),
            Stat::from(Stats::AtkPct, 46.6),
            Stat::from(Stats::DefPct, 58.3),
            Stat::from(Stats::EM, 186.5),
            Stat::from(Stats::ElementalDMG, 46.6),
            Stat::from(Stats::OtherDMG, 46.6),
            Stat::from(Stats::PhysicalDMG, 58.3),
        ],
        Types::Circlet => vec![
            Stat::from(Stats::HpPct, 46.6),
            Stat::from(Stats::AtkPct, 46.6),
            Stat::from(Stats::DefPct, 58.3),
            Stat::from(Stats::EM, 186.5),
            Stat::from(Stats::CR, 31.1),
            Stat::from(Stats::CD, 62.2),
            Stat::from(Stats::HB, 35.9),
        ],
    };

    match stats_values.iter().find(|s| s.stat() == main_stat) {
        Some(v) => *v.value(),
        None => panic!("ERROR: not {main_stat:?} value founded for {slot:?} stats pool"),
    }
}

/// Return a random amount of [`Sub Stats`] (3 or 4) of random sub_stats
pub fn sub_stats(rng: &mut impl Rng, main_stat: &Stats, strongbox: bool) -> [Option<Stat>; 4] {
    let mut available_stats = vec![
        WeightedStat(Stats::Hp, 6),
        WeightedStat(Stats::Atk, 6),
        WeightedStat(Stats::Def, 6),
        WeightedStat(Stats::HpPct, 4),
        WeightedStat(Stats::AtkPct, 4),
        WeightedStat(Stats::DefPct, 4),
        WeightedStat(Stats::ER, 4),
        WeightedStat(Stats::EM, 4),
        WeightedStat(Stats::CR, 3),
        WeightedStat(Stats::CD, 3),
    ];
    available_stats.retain(|s| &s.0 != main_stat);

    let mut substats: [Option<Stat>; 4] = [None, None, None, None];
    let substats_amount = get_substats_amount(rng, strongbox);

    for i in 0..substats_amount as usize {
        let sub_stat = generate_substat(rng, &mut available_stats);
        match substats.get_mut(i) {
            Some(op) => *op = Some(sub_stat),
            None => panic!("ERROR: failed at indexing substats"),
        };
    }

    substats
}

fn generate_substat(rng: &mut impl Rng, available_stats: &mut Vec<WeightedStat>) -> Stat {
    let weights = available_stats.iter().map(|s| s.1);
    let dist = match WeightedIndex::new(weights) {
        Ok(di) => di,
        Err(_) => panic!("ERROR: failed at mapping weights values"),
    };
    let index = dist.sample(rng);

    let stat = match available_stats.get(index) {
        Some(s) => s.0.clone(),
        None => panic!("ERROR: failed at indexing available stats"),
    };
    let value = get_substat_value(rng, &stat);
    available_stats.retain(|ws| ws.0 != stat);

    Stat::from(stat, value)
}

/// For a [`Sub Stat`](Stat) its initial value can vary from the maximum possible in relation to
/// `100%`, `90%`, `80%` and `70%` all having the same probability.
/// [Source](<https://genshin-impact.fandom.com/wiki/Artifact/Distribution#Sub_Stat_Value>)
fn get_substat_value(rng: &mut impl Rng, stat: &Stats) -> f64 {
    let value = match stat {
        Stats::Hp => 298.75,
        Stats::Atk => 19.45,
        Stats::Def => 23.15,
        Stats::HpPct => 5.83,
        Stats::AtkPct => 5.83,
        Stats::DefPct => 7.29,
        Stats::EM => 23.31,
        Stats::ER => 6.48,
        Stats::CR => 3.89,
        Stats::CD => 7.77,
        _ => panic!("ERROR {stat:?} value not found"),
    };

    let values = [value, value * 0.9, value * 0.8, value * 0.7];
    match values.choose(rng) {
        Some(v) => *v,
        None => panic!("ERROR: failed at getting random substat value for {stat:?}"),
    }
}

fn get_substats_amount(rng: &mut impl Rng, strongbox: bool) -> u8 {
    let weights = match strongbox {
        true => vec![2, 1],
        false => vec![4, 1],
    };
    let dist = match WeightedIndex::new(weights) {
        Ok(i) => i,
        Err(_) => panic!("ERROR: failed at mapping weights values"),
    };
    let index = dist.sample(rng);

    match [3u8, 4u8].get(index) {
        Some(v) => *v,
        None => panic!("ERROR: failed getting the amount of sub stats"),
    }
}
