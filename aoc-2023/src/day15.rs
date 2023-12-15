use std::collections::HashMap;

use regex::Regex;

type Steps = Vec<Step>;
type Boxes = HashMap<u32, Vec<Step>>;

#[derive(Debug, Clone)]
struct Step {
    raw: String,
    label: String,
    operation: Operation,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Dash,
    Equals(u32),
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day15.txt", true).collect();

    let steps = parse_steps(&lines);

    let sum: u32 = steps.iter().map(|step| compute_hash(&step.raw)).sum();

    let boxes = compute_hashmap(&steps);
    let power = compute_focusing_power(&boxes);

    println!("{}", sum);
    println!("{}", power);
}

fn parse_steps(lines: &[String]) -> Steps {
    let re = Regex::new(r"(?P<label>\w+)(?P<operation>[-=])(?P<length>\d+)?").expect("regex");

    lines
        .iter()
        .flat_map(|line| {
            line.split(',')
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
        })
        .map(|part| {
            let caps = re.captures(part).expect("captures");

            let raw = part.to_string();
            let label = caps["label"].to_string();
            let operation = &caps["operation"];

            let operation = if operation == "=" {
                let length = caps["length"].to_string().parse().expect("length");

                Operation::Equals(length)
            } else {
                Operation::Dash
            };

            Step {
                raw,
                label,
                operation,
            }
        })
        .collect()
}

fn compute_hash(value: &str) -> u32 {
    let mut current = 0;

    for char in value.chars() {
        current += char as u32;
        current *= 17;
        current %= 256;
    }

    current
}

fn compute_hashmap(steps: &[Step]) -> Boxes {
    let mut boxes = Boxes::new();

    for step in steps {
        let hash = compute_hash(&step.label);
        let lenses = boxes.entry(hash).or_default();
        let index = lenses.iter().position(|lens| lens.label == step.label);

        match step.operation {
            Operation::Dash => {
                if let Some(index) = index {
                    lenses.remove(index);
                }
            }
            Operation::Equals(_) => {
                if let Some(index) = index {
                    lenses.remove(index);
                    lenses.insert(index, step.clone());
                } else {
                    lenses.push(step.clone());
                }
            }
        }
    }

    boxes.retain(|_, lenses| !lenses.is_empty());

    boxes
}

fn compute_focusing_power(boxes: &Boxes) -> u32 {
    boxes
        .iter()
        .flat_map(|(box_id, box_lenses)| {
            box_lenses.iter().enumerate().map(move |(index, lens)| {
                let lens_slot = index as u32 + 1;

                let lens_length = match lens.operation {
                    Operation::Equals(length) => length,
                    _ => unreachable!(),
                };

                (1 + box_id) * lens_slot * lens_length
            })
        })
        .sum()
}
