use aoc_utils;
use rayon::prelude::*;
use std::collections::HashMap;

type Fuel = usize;
type Position = usize;
type Count = usize;
type Cluster = (Position, Count);
type Clusters = Vec<Cluster>;

pub fn run() {
    let numbers: Vec<usize> = aoc_utils::read_numbers("inputs/day07.txt", ",").collect();

    let clusters = parse_clusters(numbers);

    let fuel_cost_constant = least_fuel_cost(&clusters, true);
    let fuel_cost_increasing = least_fuel_cost(&clusters, false);

    println!("{}", fuel_cost_constant);
    println!("{}", fuel_cost_increasing);
}

fn parse_clusters(numbers: Vec<usize>) -> Clusters {
    let mut map: HashMap<Position, Count> = HashMap::new();

    for number in numbers {
        map.entry(number)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    map.into_iter().collect()
}

fn least_fuel_cost(clusters: &Clusters, constant_rate: bool) -> Fuel {
    let min = *clusters.iter().map(|(x, _)| x).min().expect("min");
    let max = *clusters.iter().map(|(x, _)| x).max().expect("max");

    (min..max + 1)
        .into_par_iter()
        .map(|next_position| fuel_cost_clusters(clusters, next_position, constant_rate))
        .min()
        .expect("min")
}

fn fuel_cost_clusters(clusters: &Clusters, next: Position, constant: bool) -> Fuel {
    clusters
        .par_iter()
        .map(|&(current, count)| fuel_cost(current, next, constant) * count)
        .sum()
}

fn fuel_cost(current: Position, next: Position, constant: bool) -> Fuel {
    let distance = (next as isize - current as isize).abs() as usize;

    if constant {
        distance
    } else {
        ((distance + 1) * distance) / 2
    }
}
