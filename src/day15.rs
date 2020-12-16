use super::utils;
use std::collections::HashMap;

pub fn run() {
    let numbers: Vec<u128> = utils::read_numbers("inputs/day15.txt", ",").collect();

    let number_2020 = find_nth_number(&numbers, 2020);
    let number_30000000 = find_nth_number(&numbers, 30000000);

    println!("{}", number_2020);
    println!("{}", number_30000000);
}

fn find_nth_number(numbers: &Vec<u128>, nth: u128) -> u128 {
    let mut last_number = 0;
    let mut last_turn: HashMap<u128, u128> = HashMap::new();
    let mut before_last_turn: HashMap<u128, u128> = HashMap::new();

    for (index, number) in numbers.iter().enumerate() {
        last_number = *number;
        last_turn.insert(last_number, index as u128 + 1);
    }

    for turn in (numbers.len() + 1) as u128..nth as u128 + 1 {
        if let (Some(_), None) = (
            last_turn.get(&last_number),
            before_last_turn.get(&last_number),
        ) {
            last_number = 0;

            update_turn_history(last_number, turn, &mut last_turn, &mut before_last_turn);
        } else if let (Some(last_number_turn), Some(before_last_number_turn)) = (
            last_turn.get(&last_number),
            before_last_turn.get(&last_number),
        ) {
            last_number = last_number_turn - before_last_number_turn;

            update_turn_history(last_number, turn, &mut last_turn, &mut before_last_turn);
        }
    }

    last_number
}

fn update_turn_history(
    number: u128,
    turn: u128,
    last_turn: &mut HashMap<u128, u128>,
    before_last_turn: &mut HashMap<u128, u128>,
) {
    if let Some(last_number_turn) = last_turn.get(&number) {
        before_last_turn
            .entry(number)
            .and_modify(|value| *value = *last_number_turn)
            .or_insert(*last_number_turn);
    }

    last_turn
        .entry(number)
        .and_modify(|value| *value = turn)
        .or_insert(turn);
}
