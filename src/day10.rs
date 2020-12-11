use super::utils;
use rayon::prelude::*;
use std::collections::HashMap;

const MAX: u128 = 3;
const OUTLET: u128 = 0;
const DEVICE: u128 = 3;

pub fn run() {
    let adapters: Vec<u128> = utils::read_lines("inputs/day10.txt", true)
        .map(|line| line.parse().expect("number"))
        .collect();

    let differences = find_differences(&adapters);

    let count1 = differences.get(&1).unwrap_or(&0);
    let count3 = differences.get(&3).unwrap_or(&0);
    let count_differences = count1 * count3;

    let mut cache: HashMap<u128, u128> = HashMap::new();
    let arrangements = find_arrangements(&adapters, 0, &mut cache);

    println!("{:?}", count_differences);
    println!("{:?}", arrangements);
}

fn find_differences(adapters: &[u128]) -> HashMap<u128, u128> {
    let mut differences: HashMap<u128, u128> = HashMap::new();

    let mut joltage = Some(OUTLET);

    while let Some(jolts) = joltage {
        let next_adapter = find_next_adapters(adapters, jolts)
            .par_iter()
            .min()
            .map(|adapter| *adapter);

        if let Some(adapter) = next_adapter {
            let difference = adapter - jolts;

            differences
                .entry(difference)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        joltage = next_adapter;
    }

    differences
        .entry(DEVICE)
        .and_modify(|count| *count += 1)
        .or_insert(1);

    differences
}

fn find_arrangements(adapters: &[u128], jolts: u128, cache: &mut HashMap<u128, u128>) -> u128 {
    let next_adapters = find_next_adapters(adapters, jolts);

    if next_adapters.is_empty() {
        1
    } else {
        next_adapters
            .into_iter()
            .map(|next_adapter| {
                cache
                    .get(&next_adapter)
                    .map(|count| *count)
                    .unwrap_or_else(|| {
                        let count = find_arrangements(adapters, next_adapter, cache);

                        cache.insert(next_adapter, count);

                        count
                    })
            })
            .sum()
    }
}

fn find_next_adapters(adapters: &[u128], jolts: u128) -> Vec<u128> {
    adapters
        .par_iter()
        .filter(|adapter| **adapter > jolts && **adapter <= jolts + MAX)
        .map(|adapter| *adapter)
        .collect()
}
