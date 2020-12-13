use super::utils;
use rayon::prelude::*;

#[derive(Debug)]
struct Instruction {
    action: String,
    value: usize,
}

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Vector {
        Vector { x: x, y: y }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate(&self, degrees: i32) -> Vector {
        match degrees {
            90 | -270 => Vector::new(-self.y, self.x),
            180 | -180 => Vector::new(-self.x, -self.y),
            270 | -90 => Vector::new(self.y, -self.x),
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    ship_translation: Vector,
    waypoint_translation: Vector,
}

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day12.txt", true).collect();

    let instructions = parse_instructions(&lines);

    let state = follow_instructions(&instructions, false);
    let state_waypoint = follow_instructions(&instructions, true);

    println!("{}", state.ship_translation.manhattan_distance());
    println!("{}", state_waypoint.ship_translation.manhattan_distance());
}

fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    lines
        .par_iter()
        .map(|line| Instruction {
            action: (&line[0..1]).to_string(),
            value: (&line[1..]).to_string().parse().expect("number"),
        })
        .collect()
}

fn follow_instructions(instructions: &Vec<Instruction>, waypoint: bool) -> State {
    let mut state = if waypoint {
        State {
            ship_translation: Vector::new(0, 0),
            waypoint_translation: Vector::new(10, 1),
        }
    } else {
        State {
            ship_translation: Vector::new(0, 0),
            waypoint_translation: Vector::new(1, 0),
        }
    };

    for instruction in instructions {
        state = follow_instruction(instruction, &state, waypoint);
    }

    state
}

fn follow_instruction(instruction: &Instruction, current: &State, waypoint: bool) -> State {
    let mut next = current.clone();

    if waypoint {
        match &instruction.action[..] {
            "N" => {
                next.waypoint_translation.y =
                    current.waypoint_translation.y + instruction.value as i32;
            }
            "S" => {
                next.waypoint_translation.y =
                    current.waypoint_translation.y - instruction.value as i32;
            }
            "E" => {
                next.waypoint_translation.x =
                    current.waypoint_translation.x + instruction.value as i32;
            }
            "W" => {
                next.waypoint_translation.x =
                    current.waypoint_translation.x - instruction.value as i32;
            }
            _ => (),
        }
    } else {
        match &instruction.action[..] {
            "N" => {
                next.ship_translation.y = current.ship_translation.y + instruction.value as i32;
            }
            "S" => {
                next.ship_translation.y = current.ship_translation.y - instruction.value as i32;
            }
            "E" => {
                next.ship_translation.x = current.ship_translation.x + instruction.value as i32;
            }
            "W" => {
                next.ship_translation.x = current.ship_translation.x - instruction.value as i32;
            }
            _ => (),
        }
    }

    match &instruction.action[..] {
        "L" => {
            next.waypoint_translation = current
                .waypoint_translation
                .rotate(instruction.value as i32);
        }
        "R" => {
            next.waypoint_translation = current
                .waypoint_translation
                .rotate(instruction.value as i32 * -1);
        }
        "F" => {
            next.ship_translation.x = current.ship_translation.x
                + (current.waypoint_translation.x * instruction.value as i32);

            next.ship_translation.y = current.ship_translation.y
                + (current.waypoint_translation.y * instruction.value as i32);
        }
        _ => (),
    }

    next
}
