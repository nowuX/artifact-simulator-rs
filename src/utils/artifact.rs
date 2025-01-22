use crate::utils::WeightedStat;
use crate::{Stat, Stats, SubStats, Types};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::SliceRandom;

/// Return a random [`Stat`] for the artifact main stat
pub fn main_stat(slot: &Types) -> Stat {
    let main_stat = get_main_stat(slot);
    let main_value = get_main_value(slot, &main_stat);

    Stat::new(main_stat, main_value)
}

/// Returns a random [`Stat`][Stats] for the specific artifact [`Type`](Types)
fn get_main_stat(slot: &Types) -> Stats {
    let stats_weights = match slot {
        Types::Flower => Vec::from([WeightedStat::new(Stats::Hp, 1)]),
        Types::Plume => Vec::from([WeightedStat::new(Stats::Atk, 1)]),
        Types::Sand => Vec::from([
            WeightedStat::new(Stats::HpPct, 8),
            WeightedStat::new(Stats::AtkPct, 8),
            WeightedStat::new(Stats::DefPct, 8),
            WeightedStat::new(Stats::EM, 3),
            WeightedStat::new(Stats::ER, 3),
        ]),
        Types::Goblet => Vec::from([
            WeightedStat::new(Stats::HpPct, 77),
            WeightedStat::new(Stats::AtkPct, 77),
            WeightedStat::new(Stats::DefPct, 76),
            WeightedStat::new(Stats::ElementalDMG, 20),
            WeightedStat::new(Stats::PhysicalDMG, 20),
            WeightedStat::new(Stats::OtherDMG, 120),
            WeightedStat::new(Stats::EM, 10),
        ]),
        Types::Circlet => Vec::from([
            WeightedStat::new(Stats::HpPct, 11),
            WeightedStat::new(Stats::AtkPct, 11),
            WeightedStat::new(Stats::DefPct, 11),
            WeightedStat::new(Stats::CD, 5),
            WeightedStat::new(Stats::CR, 5),
            WeightedStat::new(Stats::HB, 5),
            WeightedStat::new(Stats::EM, 2),
        ]),
    };

    if stats_weights.len() == 1 {
        return stats_weights.first().unwrap().stat;
    }

    let weights = stats_weights.iter().map(|x| x.weight);
    let dist = WeightedIndex::new(weights).unwrap();
    let index = dist.sample(&mut rand::thread_rng());

    stats_weights[index].stat
}

fn get_main_value(r#type: &Types, main_stat: &Stats) -> f64 {
    let stats_weights = match r#type {
        Types::Flower => Vec::from([Stat::new_i32(Stats::Hp, 4780)]),
        Types::Plume => Vec::from([Stat::new_i32(Stats::Atk, 311)]),
        Types::Sand => Vec::from([
            Stat::new(Stats::HpPct, 46.6),
            Stat::new(Stats::AtkPct, 46.6),
            Stat::new(Stats::DefPct, 58.3),
            Stat::new(Stats::ER, 51.8),
            Stat::new(Stats::EM, 186.5),
        ]),
        Types::Goblet => Vec::from([
            Stat::new(Stats::HpPct, 46.6),
            Stat::new(Stats::AtkPct, 46.6),
            Stat::new(Stats::DefPct, 58.3),
            Stat::new(Stats::EM, 186.5),
            Stat::new(Stats::ElementalDMG, 46.6),
            Stat::new(Stats::OtherDMG, 46.6),
            Stat::new(Stats::PhysicalDMG, 58.3),
        ]),
        Types::Circlet => Vec::from([
            Stat::new(Stats::HpPct, 46.6),
            Stat::new(Stats::AtkPct, 46.6),
            Stat::new(Stats::DefPct, 58.3),
            Stat::new(Stats::EM, 186.5),
            Stat::new(Stats::CR, 31.1),
            Stat::new(Stats::CD, 62.2),
            Stat::new(Stats::HB, 35.9),
        ]),
    };

    if stats_weights.len() == 1 {
        return stats_weights.first().unwrap().value;
    }

    match stats_weights.iter().find(|s| &s.stat == main_stat) {
        None => panic!("Value for stat {main_stat}"),
        Some(s) => s.value,
    }
}

/// Return a random amount of [`Sub Stats`] (3 or 4) of random sub_stats
pub fn sub_stats(main_stat: &Stats, strongbox: bool) -> SubStats {
    let mut available_stats = Vec::from([
        WeightedStat::new(Stats::Hp, 6),
        WeightedStat::new(Stats::Atk, 6),
        WeightedStat::new(Stats::Def, 6),
        WeightedStat::new(Stats::HpPct, 4),
        WeightedStat::new(Stats::AtkPct, 4),
        WeightedStat::new(Stats::DefPct, 4),
        WeightedStat::new(Stats::ER, 4),
        WeightedStat::new(Stats::EM, 4),
        WeightedStat::new(Stats::CR, 3),
        WeightedStat::new(Stats::CD, 3),
    ]);
    available_stats.retain(|s| &s.stat != main_stat);

    let mut sub_stats = SubStats::default();
    let num_sub_stats = get_num_sub_stats(strongbox);

    for _ in 0..num_sub_stats {
        let sub_stat = get_sub_stat(&mut available_stats);
        sub_stats.add_stat(sub_stat);
    }

    sub_stats
}

fn get_sub_stat(available_stats: &mut Vec<WeightedStat>) -> Stat {
    let weights = available_stats.iter().map(|s| s.weight).collect();
    let dist = get_weight_index(weights);
    let index = dist.sample(&mut rand::thread_rng());

    let stat = available_stats[index].stat;
    let value = sub_stat_value(&stat);
    available_stats.retain(|s| s.stat != stat);

    Stat::new(stat, value)
}

/// For a [`Sub Stat`](Stat) its initial value can vary from the maximum possible in relation to
/// `100%`, `90%`, `80%` and `70%` all having the same probability.
/// [Source](<https://genshin-impact.fandom.com/wiki/Artifact/Distribution#Sub_Stat_Value>)
fn sub_stat_value(stat: &Stats) -> f64 {
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
        _ => panic!("{stat} value not found"),
    };

    let values = Vec::from([value, value * 0.9, value * 0.8, value * 0.7]);
    match values.choose(&mut rand::thread_rng()) {
        Some(v) => v.to_owned(),
        None => panic!("Error getting random `Sub Stat` value for {stat}"),
    }
}

fn get_num_sub_stats(strongbox: bool) -> usize {
    let initial_sub_stats = [3, 4];
    let weights = match strongbox {
        true => [2, 1],
        false => [4, 1],
    };
    let dist = get_weight_index(weights.to_vec());
    let index = dist.sample(&mut rand::thread_rng());

    initial_sub_stats[index]
}

fn get_weight_index(weights: Vec<i32>) -> WeightedIndex<i32> {
    match WeightedIndex::new(weights) {
        Ok(i) => i,
        Err(e) => panic!("Error in weights:\n{}", e),
    }
}
