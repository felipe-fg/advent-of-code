use std::env;

mod day01;
mod day02;

fn main() {
    let day: i32 = env::args().nth(1).expect("day").parse().expect("number");

    match day {
        1 => day01::run(),
        2 => day02::run(),
        _ => println!("invalid day"),
    }
}
