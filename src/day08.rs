use super::utils;
use std::collections::HashSet;

type ID = usize;
type Operation = String;
type Argument = i32;
type Instruction = (ID, Operation, Argument);

type Accumulator = i32;
type Exit = i32;
type Status = (Exit, Accumulator);

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day08.txt", true).collect();

    let instructions = parse_instructions(lines);

    let (_, accumulator_loop) = execute(&instructions);
    let (_, accumulator_fixed) = brute_force(&instructions);

    println!("{}", accumulator_loop);
    println!("{}", accumulator_fixed);
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .enumerate()
        .map(|(id, line)| {
            let mut parts = line.split_whitespace();

            let operation = parts.next().expect("operation").to_string();
            let argument = parts.next().expect("argument").parse().expect("number");

            (id, operation, argument)
        })
        .collect()
}

fn execute(instructions: &Vec<Instruction>) -> Status {
    let mut processed = HashSet::new();

    let mut position = 0;
    let mut accumulator = 0;
    let mut exit = 0;

    while let Some((id, operation, argument)) = instructions.get(position as usize) {
        if processed.contains(id) {
            exit = -1;
            break;
        }

        match &operation[..] {
            "jmp" => position += argument,
            "acc" => {
                accumulator += argument;
                position += 1;
            }
            _ => position += 1,
        };

        processed.insert(id);
    }

    (exit, accumulator)
}

fn brute_force(instructions: &Vec<Instruction>) -> Status {
    for (fix_id, fix_operation, _) in instructions {
        let fix_operation = match fix_operation.as_str() {
            "jmp" => String::from("nop"),
            "nop" => String::from("jmp"),
            _ => continue,
        };

        let instructions: Vec<Instruction> = instructions
            .iter()
            .map(|(id, operation, argument)| match id == fix_id {
                true => (*id, fix_operation.to_string(), *argument),
                false => (*id, operation.to_string(), *argument),
            })
            .collect();

        let status = execute(&instructions);
        let (exit, _) = status;

        if exit == 0 {
            return status;
        }
    }

    execute(instructions)
}
