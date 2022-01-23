use aoc_utils;
use std::collections::HashSet;

type Value = i64;

type Digit = Value;
type Z = Value;
type ID = (Digit, Z);
type IDSet = HashSet<ID>;

const DIGIT_COUNT: Value = 14;
const DIGIT_VALID: Value = 0;

const MODEL_START: Value = 1;
const MODEL_END: Value = 9;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operand {
    Constant(Value),
    Register(Register),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct Memory {
    w: Value,
    x: Value,
    y: Value,
    z: Value,
}

impl Memory {
    fn read(&self, register: Register) -> Value {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn write(&mut self, register: Register, value: Value) {
        match register {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ALU {
    memory: Memory,
    instructions: Vec<Instruction>,
}

impl ALU {
    fn new(instructions: Vec<Instruction>) -> Self {
        let memory = Memory::default();

        Self {
            memory,
            instructions,
        }
    }

    fn run(&mut self, input: Vec<Value>) -> bool {
        let mut input = input.into_iter();
        let mut next = 0;

        for (current, &instruction) in self.instructions.iter().enumerate() {
            match instruction {
                Instruction::Inp(a) => {
                    if let Some(value) = input.next() {
                        self.memory.write(a, value as Value);
                    } else {
                        break;
                    }
                }
                Instruction::Add(a, b) => {
                    let value = self.memory.read(a) + self.operand(b);

                    self.memory.write(a, value as Value);
                }
                Instruction::Mul(a, b) => {
                    let value = self.memory.read(a) * self.operand(b);

                    self.memory.write(a, value as Value);
                }
                Instruction::Div(a, b) => {
                    let value = self.memory.read(a) / self.operand(b);

                    self.memory.write(a, value as Value);
                }
                Instruction::Mod(a, b) => {
                    let value = self.memory.read(a) % self.operand(b);

                    self.memory.write(a, value as Value);
                }
                Instruction::Eql(a, b) => {
                    let value = self.memory.read(a) == self.operand(b);

                    self.memory.write(a, value as Value);
                }
            }

            next = current + 1;
        }

        self.instructions.drain(0..next);

        self.instructions.is_empty()
    }

    fn operand(&self, operand: Operand) -> Value {
        match operand {
            Operand::Constant(value) => value,
            Operand::Register(register) => self.memory.read(register),
        }
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day24.txt", true).collect();

    let instructions = parse_instructions(lines);

    let largest = compute_model_number(instructions.clone(), true);
    let smallest = compute_model_number(instructions.clone(), false);

    println!("{}", largest);
    println!("{}", smallest);
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines.iter().map(|line| parse_instruction(line)).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let mut parts = line.split_ascii_whitespace();

    let instruction = parts.next().expect("instruction");
    let a = parts.next();
    let b = parts.next();

    let a = || a.map(parse_register).expect("a");
    let b = || b.map(parse_operand).expect("b");

    match instruction {
        "inp" => Instruction::Inp(a()),
        "add" => Instruction::Add(a(), b()),
        "mul" => Instruction::Mul(a(), b()),
        "div" => Instruction::Div(a(), b()),
        "mod" => Instruction::Mod(a(), b()),
        "eql" => Instruction::Eql(a(), b()),
        _ => panic!(),
    }
}

fn parse_register(value: &str) -> Register {
    match value {
        "w" => Register::W,
        "x" => Register::X,
        "y" => Register::Y,
        "z" => Register::Z,
        _ => panic!(),
    }
}

fn parse_operand(value: &str) -> Operand {
    match value {
        "w" | "x" | "y" | "z" => Operand::Register(parse_register(value)),
        _ => Operand::Constant(value.parse().expect("value")),
    }
}

fn compute_model_number(instructions: Vec<Instruction>, largest: bool) -> String {
    fn compute_loop(alu: &ALU, digit: Value, largest: bool, dedup: &mut IDSet) -> Option<String> {
        let mut iter: Box<dyn Iterator<Item = Value>> = if largest {
            Box::new((MODEL_START..=MODEL_END).rev())
        } else {
            Box::new(MODEL_START..=MODEL_END)
        };

        iter.find_map(|input| {
            let mut alu = alu.clone();

            alu.run(vec![input]);

            let is_last = digit == DIGIT_COUNT;

            if is_last {
                let is_valid = alu.memory.z == DIGIT_VALID;

                if is_valid {
                    return Some(input.to_string());
                }
            } else {
                let id = (digit, alu.memory.z);

                let is_unique = !dedup.contains(&id);

                if is_unique {
                    dedup.insert(id);

                    let next = compute_loop(&alu, digit + 1, largest, dedup);

                    return next.map(|next| format!("{}{}", input, next));
                }
            }

            None
        })
    }

    compute_loop(&ALU::new(instructions), 1, largest, &mut IDSet::new()).expect("model")
}
