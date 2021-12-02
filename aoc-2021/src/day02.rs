use aoc_utils;
use rayon::prelude::*;

type Direction = String;
type Value = i32;
type Command = (Direction, Value);
type Horizontal = Value;
type Depth = Value;
type Aim = Value;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day02.txt", true).collect();

    let commands = parse_commands(lines);

    let (one_horizontal, one_depth) = move_submarine(&commands);
    let (two_horizontal, two_depth, _) = move_aim_submarine(&commands);

    println!("{}", one_horizontal * one_depth);
    println!("{}", two_horizontal * two_depth);
}

fn parse_commands(lines: Vec<String>) -> Vec<Command> {
    lines.par_iter().map(|line| parse_command(line)).collect()
}

fn parse_command(line: &str) -> Command {
    let mut parts = line.split_ascii_whitespace();

    let direction = parts.next().expect("direction").trim().to_string();
    let value = parts.next().expect("value").trim().parse().expect("number");

    (direction, value)
}

fn move_submarine(commands: &[Command]) -> (Horizontal, Depth) {
    commands.iter().fold(
        (0, 0),
        |(horizontal, depth), (direction, value)| match &direction[..] {
            "forward" => (horizontal + value, depth),
            "down" => (horizontal, depth + value),
            "up" => (horizontal, depth - value),
            _ => (horizontal, depth),
        },
    )
}

fn move_aim_submarine(commands: &[Command]) -> (Horizontal, Depth, Aim) {
    commands.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), (direction, value)| match &direction[..] {
            "down" => (horizontal, depth, aim + value),
            "up" => (horizontal, depth, aim - value),
            "forward" => (horizontal + value, depth + (aim * value), aim),
            _ => (horizontal, depth, aim),
        },
    )
}
