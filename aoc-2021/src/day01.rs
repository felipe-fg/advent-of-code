use aoc_utils;
use itertools::Itertools;

pub fn run() {
    let numbers: Vec<i32> = aoc_utils::read_numbers("inputs/day01.txt", ",").collect();

    let count_two = numbers
        .iter()
        .tuple_windows()
        .filter(|(one, two)| two > one)
        .count();

    let count_three = numbers
        .iter()
        .tuple_windows()
        .map(|(one, two, three)| one + two + three)
        .tuple_windows()
        .filter(|(one, two)| two > one)
        .count();

    println!("{}", count_two);
    println!("{}", count_three);
}
