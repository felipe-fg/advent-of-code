use std::collections::HashMap;

use rayon::prelude::*;

type Hands = Vec<Hand>;
type Number = u128;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: Number,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day07.txt", true).collect();

    let hands = parse_hands(&lines);

    let winnings = compute_total_winnings(&hands, false);
    let winnings_joker = compute_total_winnings(&hands, true);

    println!("{}", winnings);
    println!("{}", winnings_joker);
}

fn parse_hands(lines: &[String]) -> Hands {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();

            let cards = parts.next().expect("cards").to_string();
            let bid = parts.next().expect("bid").parse().expect("bid");

            Hand { cards, bid }
        })
        .collect()
}

fn compute_total_winnings(hands: &[Hand], joker: bool) -> Number {
    let mut strengths: Vec<_> = hands
        .par_iter()
        .map(|hand| (hand, get_hand_strength(&hand.cards, joker)))
        .collect();

    strengths.sort_by_key(|(_, strength)| *strength);

    strengths
        .iter()
        .enumerate()
        .map(|(index, (hand, _))| hand.bid * (index as u128 + 1))
        .sum()
}

fn get_hand_strength(cards: &str, joker: bool) -> Number {
    let type_strength = if joker {
        let combinations = get_combinations_joker(cards);

        combinations
            .par_iter()
            .map(|combination_cards| get_type_strength(combination_cards))
            .max()
            .expect("max")
    } else {
        get_type_strength(cards)
    };

    let ordering_strength = get_ordering_strength(cards, joker);
    let type_bin = 100u128.pow(cards.len() as u32);

    (type_strength * type_bin) + ordering_strength
}

fn get_type_strength(cards: &str) -> Number {
    if is_five_of_a_kind(cards) {
        7
    } else if is_four_of_a_kind(cards) {
        6
    } else if is_full_house(cards) {
        5
    } else if is_three_of_a_kind(cards) {
        4
    } else if is_two_pair(cards) {
        3
    } else if is_one_pair(cards) {
        2
    } else {
        1
    }
}

fn get_ordering_strength(cards: &str, joker: bool) -> Number {
    cards
        .chars()
        .rev()
        .enumerate()
        .map(|(index, label)| {
            let label_strength = get_label_strength(label, joker);
            let label_bin = 100u128.pow(index as u32);

            label_strength * label_bin
        })
        .sum()
}

fn is_five_of_a_kind(cards: &str) -> bool {
    count_cards_by_label(cards)
        .iter()
        .any(|(_, &count)| count == 5)
}

fn is_four_of_a_kind(cards: &str) -> bool {
    count_cards_by_label(cards)
        .iter()
        .any(|(_, &count)| count == 4)
}

fn is_full_house(cards: &str) -> bool {
    let labels = count_cards_by_label(cards);

    labels.iter().any(|(_, &count)| count == 3) && labels.iter().any(|(_, &count)| count == 2)
}

fn is_three_of_a_kind(cards: &str) -> bool {
    let labels = count_cards_by_label(cards);

    labels.iter().any(|(_, &count)| count == 3) && labels.iter().any(|(_, &count)| count != 2)
}

fn is_two_pair(cards: &str) -> bool {
    let pairs = count_cards_by_label(cards)
        .iter()
        .filter(|(_, &count)| count == 2)
        .count();

    pairs == 2
}

fn is_one_pair(cards: &str) -> bool {
    let pairs = count_cards_by_label(cards)
        .iter()
        .filter(|(_, &count)| count == 2)
        .count();

    pairs == 1
}

fn count_cards_by_label(cards: &str) -> HashMap<char, Number> {
    let mut labels = HashMap::new();

    for card in cards.chars() {
        *labels.entry(card).or_default() += 1;
    }

    labels
}

fn get_label_strength(label: char, joker: bool) -> Number {
    match label {
        'J' if joker => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' if !joker => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => unreachable!(),
    }
}

fn get_combinations_joker(initial_cards: &str) -> Vec<String> {
    let jokers = initial_cards.chars().filter(|&label| label == 'J').count();

    let mut combinations = vec![initial_cards.to_string()];

    for _ in 0..jokers {
        let mut next_combinations = Vec::new();

        for cards in combinations {
            for label in ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
                let next_card = cards.replacen('J', &label.to_string(), 1);

                next_combinations.push(next_card);
            }
        }

        combinations = next_combinations;
    }

    combinations
}
