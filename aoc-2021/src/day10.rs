use aoc_utils;
use itertools::Itertools;
use rayon::prelude::*;

type Score = u64;

#[derive(Debug, Clone)]
struct Validation {
    full: String,
    status: Status,
    error: usize,
    stack: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Status {
    Valid,
    Incomplete,
    Corrupted,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day10.txt", true).collect();

    let validations = check_lines(&lines);

    let syntax_error = sum_syntax_error_score(&validations);
    let autocomplete = median_autocomplete_score(&validations);

    println!("{}", syntax_error);
    println!("{}", autocomplete);
}

fn check_lines(lines: &[String]) -> Vec<Validation> {
    lines.par_iter().map(|line| check_line(line)).collect()
}

fn check_line(line: &str) -> Validation {
    let mut status: Status = Status::Valid;
    let mut error: usize = 0;
    let mut stack: Vec<char> = vec![];

    for (index, symbol) in line.trim().chars().enumerate() {
        match symbol {
            '(' | '[' | '{' | '<' => stack.push(symbol),
            ')' | ']' | '}' | '>' => match stack.last() {
                None => {
                    status = Status::Corrupted;
                    error = index;
                    break;
                }
                Some(&last) if invert(last) != symbol => {
                    status = Status::Corrupted;
                    error = index;
                    break;
                }
                _ => {
                    stack.pop();
                }
            },
            _ => (),
        }
    }

    if status == Status::Valid && !stack.is_empty() {
        status = Status::Incomplete
    }

    Validation {
        full: line.to_string(),
        status,
        error,
        stack: stack.iter().join(""),
    }
}

fn sum_syntax_error_score(validations: &[Validation]) -> Score {
    validations
        .iter()
        .filter(|validation| validation.status == Status::Corrupted)
        .map(|validation| syntax_error_score(validation))
        .sum()
}

fn syntax_error_score(validation: &Validation) -> Score {
    let error = validation
        .full
        .chars()
        .nth(validation.error)
        .expect("error");

    match error {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn median_autocomplete_score(validations: &[Validation]) -> Score {
    let scores: Vec<Score> = validations
        .iter()
        .filter(|validation| validation.status == Status::Incomplete)
        .map(|validation| autocomplete_score(validation))
        .sorted()
        .collect();

    scores[scores.len() / 2]
}

fn autocomplete_score(validation: &Validation) -> Score {
    validation
        .stack
        .chars()
        .map(|symbol| invert(symbol))
        .rev()
        .fold(0, |acc, symbol| {
            let score = match symbol {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            };

            acc * 5 + score
        })
}

fn invert(symbol: char) -> char {
    match symbol {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => ' ',
    }
}
