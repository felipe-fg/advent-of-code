use aoc_utils;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

type Memory = HashMap<Address, Value>;
type Address = u64;
type Value = u64;

type Program = Vec<Command>;

#[derive(Debug, Clone)]
struct Command {
    action: Action,
    bitmask: String,
    address: Address,
    value: Value,
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Bitmask,
    Value,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day14.txt", true).collect();

    let program = parse_program(&lines);

    let memory_v1 = run_program_v1(&program);
    let memory_v2 = run_program_v2(&program);

    let memory_v1_sum: u64 = memory_v1.values().sum();
    let memory_v2_sum: u64 = memory_v2.values().sum();

    println!("{:?}", memory_v1_sum);
    println!("{:?}", memory_v2_sum);
}

fn parse_program(lines: &Vec<String>) -> Program {
    let mask_re = Regex::new(r"mask = (?P<bitmask>.+)").expect("mark regex");
    let mem_re = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").expect("mem regex");

    lines
        .par_iter()
        .map(|line| {
            if line.starts_with("mask") {
                let caps = mask_re.captures(line).expect("captures");
                let bitmask = caps["bitmask"].to_string();

                Command {
                    action: Action::Bitmask,
                    bitmask: bitmask,
                    address: 0,
                    value: 0,
                }
            } else {
                let caps = mem_re.captures(line).expect("captures");
                let address = caps["address"].to_string().parse().expect("number");
                let value = caps["value"].to_string().parse().expect("number");

                Command {
                    action: Action::Value,
                    bitmask: String::from(""),
                    address: address,
                    value: value,
                }
            }
        })
        .collect()
}

fn run_program_v1(program: &Program) -> Memory {
    let mut memory = HashMap::new();
    let mut bitmask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    for command in program {
        match command.action {
            Action::Bitmask => bitmask = &command.bitmask,
            Action::Value => {
                let new_value = apply_bitmask(bitmask, command.value, "01", |_, flag| flag);

                memory
                    .entry(command.address)
                    .and_modify(|value| *value = new_value)
                    .or_insert(new_value);
            }
        }
    }

    memory
}

fn run_program_v2(program: &Program) -> Memory {
    let mut memory = HashMap::new();
    let mut bitmask = "000000000000000000000000000000000000";

    for command in program {
        match command.action {
            Action::Bitmask => bitmask = &command.bitmask,
            Action::Value => {
                let new_addresses = apply_floating(bitmask, command.address);

                for new_address in new_addresses {
                    memory
                        .entry(new_address)
                        .and_modify(|value| *value = command.value)
                        .or_insert(command.value);
                }
            }
        }
    }

    memory
}

fn apply_bitmask(
    bitmask: &str,
    number: u64,
    condition: &str,
    mut replace_flag: impl FnMut(usize, char) -> char,
) -> u64 {
    bitmask
        .chars()
        .rev()
        .enumerate()
        .filter(|(_, flag)| condition.contains(*flag))
        .fold(number, |number, (index, flag)| {
            let bit = replace_flag(index, flag) == '1';

            set_number_bit(number, bit, index)
        })
}

fn apply_floating(bitmask: &str, address: Address) -> Vec<Address> {
    let base_address = apply_bitmask(bitmask, address, "1", |_, flag| flag);

    let floating_count = bitmask.chars().filter(|flag| flag == &'X').count();

    build_binary_combinations(floating_count)
        .into_iter()
        .map(|combination| {
            let mut iter = combination.chars();

            apply_bitmask(bitmask, base_address, "X", |_, _| {
                iter.next().expect("combination")
            })
        })
        .collect()
}

fn set_number_bit(number: u64, bit: bool, index: usize) -> u64 {
    match bit {
        true => number | (1u64 << index),
        false => number & (!(1u64 << index)),
    }
}

fn build_binary_combinations(length: usize) -> Vec<String> {
    fn build(length: usize, combination: String) -> Vec<String> {
        if combination.len() == length {
            vec![combination]
        } else {
            let mut zero = build(length, format!("{}0", combination));
            let mut one = build(length, format!("{}1", combination));

            zero.append(&mut one);

            zero
        }
    }

    build(length, String::default())
}
