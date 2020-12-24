use super::utils;
use itertools;
use std::collections::HashMap;
use std::iter;

type Label = usize;
type NextLabel = usize;

pub fn run() {
    let cups: Vec<Label> = utils::read_numbers("inputs/day23.txt", "").collect();

    let cups_one = extend_cups(&cups, 0, 0);
    let cups_two = extend_cups(&cups, 10, 1000000 + 1);

    let data_one = play_game(&cups_one, 100);
    let data_two = play_game(&cups_two, 10000000);

    let (labels, _) = build_labels_stars(&data_one);
    let (_, stars) = build_labels_stars(&data_two);

    println!("{:?}", labels);
    println!("{:?}", stars);
}

fn extend_cups(cups: &[Label], start: Label, end: Label) -> Vec<Label> {
    cups.iter().map(|label| *label).chain(start..end).collect()
}

fn play_game(cups: &[Label], moves: usize) -> HashMap<Label, NextLabel> {
    let min_label = *cups.iter().min().expect("min label");
    let max_label = *cups.iter().max().expect("max label");

    let mut current_data = build_data(&cups);
    let mut current_label = cups[0];

    for _ in 0..moves {
        next_move(&mut current_data, &mut current_label, min_label, max_label);
    }

    current_data
}

fn build_data(cups: &[Label]) -> HashMap<Label, NextLabel> {
    cups.windows(2)
        .map(|window| (window[0], window[1]))
        .chain(iter::once((cups[cups.len() - 1], cups[0])))
        .collect()
}

fn next_move(
    current_data: &mut HashMap<Label, NextLabel>,
    current_label: &mut Label,
    min_label: Label,
    max_label: Label,
) {
    let remove_one = current_data[&current_label];
    let remove_two = current_data[&remove_one];
    let remove_three = current_data[&remove_two];
    let aux_next_label = current_data[&remove_three];

    current_data.remove(&remove_one);
    current_data.remove(&remove_two);
    current_data.remove(&remove_three);

    current_data
        .entry(*current_label)
        .and_modify(|label| *label = aux_next_label);

    let destination_label = find_destination(&current_data, &current_label, min_label, max_label);
    let aux_next_label = current_data[&destination_label];

    current_data
        .entry(destination_label)
        .and_modify(|label| *label = remove_one);

    current_data.entry(remove_one).or_insert(remove_two);
    current_data.entry(remove_two).or_insert(remove_three);
    current_data.entry(remove_three).or_insert(aux_next_label);

    *current_label = current_data[current_label];
}

fn find_destination(
    current_data: &HashMap<Label, NextLabel>,
    current_label: &Label,
    min_label: Label,
    max_label: Label,
) -> Label {
    let mut destination_label = current_label - 1;

    loop {
        destination_label = if destination_label < min_label {
            max_label
        } else {
            destination_label
        };

        if current_data.contains_key(&destination_label) {
            return destination_label;
        }

        destination_label -= 1;
    }
}

fn build_cups(data: &HashMap<Label, NextLabel>) -> Vec<Label> {
    let mut cups = vec![1];

    let mut current_label = data[&1];

    while current_label != 1 {
        cups.push(current_label);

        current_label = data[&current_label];
    }

    cups
}

fn build_labels_stars(data: &HashMap<Label, NextLabel>) -> (String, u128) {
    let mut cups = build_cups(data);

    cups.remove(0);

    let stars = cups[0] as u128 * cups[1] as u128;
    let labels = itertools::join(cups, "");

    (labels, stars)
}
