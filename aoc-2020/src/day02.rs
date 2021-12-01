use aoc_utils;
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
struct Entry {
    pub left: usize,
    pub right: usize,
    pub letter: String,
    pub password: String,
}

impl Entry {
    const RE_ENTRY: &'static str = r"(?P<left>\d+)-(?P<right>\d+) (?P<letter>\w): (?P<password>.*)";

    pub fn from_line(line: &str) -> Entry {
        let caps = Regex::new(Entry::RE_ENTRY)
            .expect("regex")
            .captures(line)
            .expect("captures");

        Entry {
            left: caps["left"].to_string().parse().expect("number"),
            right: caps["right"].to_string().parse().expect("number"),
            letter: caps["letter"].to_string(),
            password: caps["password"].to_string(),
        }
    }

    pub fn is_valid_sled(self: &Entry) -> bool {
        let count = self.password.matches(&self.letter).count();

        count >= self.left && count <= self.right
    }

    pub fn is_valid_toboggan(self: &Entry) -> bool {
        let first = self.password.chars().nth(self.left - 1);
        let second = self.password.chars().nth(self.right - 1);

        if let (Some(first), Some(second)) = (first, second) {
            let first_contains = first.to_string() == self.letter;
            let second_contains = second.to_string() == self.letter;

            first_contains ^ second_contains
        } else {
            true
        }
    }
}

pub fn run() {
    let entries: Vec<Entry> = aoc_utils::read_lines("inputs/day02.txt", true)
        .collect::<Vec<String>>()
        .par_iter()
        .map(|line| Entry::from_line(&line))
        .collect();

    let count_sled = entries.iter().filter(|entry| entry.is_valid_sled()).count();

    let count_toboggan = entries
        .iter()
        .filter(|entry| entry.is_valid_toboggan())
        .count();

    println!("{}", count_sled);
    println!("{}", count_toboggan);
}
