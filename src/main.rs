use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod utils;

fn main() {
    let day: i32 = env::args().nth(1).expect("day").parse().expect("number");

    match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        6 => day06::run(),
        _ => println!("invalid day"),
    }
}
