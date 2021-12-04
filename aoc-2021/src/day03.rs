use aoc_utils;
use itertools::Itertools;
use rayon::prelude::*;

type Bit = bool;
type BitVec = Vec<Bit>;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day03.txt", true).collect();

    let items = parse_items(lines);

    let (gamma, epsilon) = get_gamma_epsilon(&items);
    let (oxygen_generator, co2_scrubber) = get_oxygen_generator_co2_scrubber(&items);

    println!("{}", gamma * epsilon);
    println!("{}", oxygen_generator * co2_scrubber);
}

fn parse_items(lines: Vec<String>) -> Vec<BitVec> {
    lines
        .par_iter()
        .map(|line| {
            line.split("")
                .filter(|bit| !bit.is_empty())
                .map(|bit| bit.parse::<usize>().expect("bit") != 0)
                .collect()
        })
        .collect()
}

fn get_gamma_epsilon(items: &[BitVec]) -> (usize, usize) {
    let most_common = get_frequency_bit_vec(items, true);
    let least_common = get_frequency_bit_vec(items, false);

    let gamma = bit_vec2usize(&most_common);
    let epsilon = bit_vec2usize(&least_common);

    (gamma, epsilon)
}

fn get_oxygen_generator_co2_scrubber(items: &[BitVec]) -> (usize, usize) {
    let most_common = get_filter_out_bit_vec(items, true);
    let least_common = get_filter_out_bit_vec(items, false);

    let oxygen_generator = bit_vec2usize(&most_common);
    let co2_scrubber = bit_vec2usize(&least_common);

    (oxygen_generator, co2_scrubber)
}

fn get_frequency_bit_vec<T: AsRef<BitVec>>(items: &[T], most_common: bool) -> BitVec {
    let dimensions = items.first().expect("first").as_ref().len();

    (0..dimensions)
        .map(|index| {
            items
                .iter()
                .map(|bit_vec| bit_vec.as_ref()[index])
                .map(|bit| match bit {
                    false => -1,
                    true => 1,
                })
                .sum::<isize>()
        })
        .map(|sum| match sum >= 0 {
            true => most_common,
            false => !most_common,
        })
        .collect()
}

fn get_filter_out_bit_vec(items: &[BitVec], most_common: bool) -> BitVec {
    let dimensions = items.first().expect("first").len();

    let mut remaining_items: Vec<&BitVec> = items.iter().collect();

    for index in 0..dimensions {
        let frequency_bit_vec = get_frequency_bit_vec(&remaining_items, most_common);

        remaining_items = remaining_items
            .into_iter()
            .filter(|bit_vec| bit_vec[index] == frequency_bit_vec[index])
            .collect();

        if remaining_items.len() <= 1 {
            break;
        }
    }

    remaining_items.first().expect("first").clone().to_owned()
}

fn bit_vec2usize(bit_vec: &BitVec) -> usize {
    let binary = bit_vec.iter().map(|&bit| bit as usize).join("");

    usize::from_str_radix(&binary[..], 2).expect("binary")
}
