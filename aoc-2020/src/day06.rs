use aoc_utils;
use std::collections::HashMap;
use std::collections::HashSet;

type Group<'a> = Vec<Person<'a>>;
type Person<'a> = Vec<Answer<'a>>;
type Answer<'a> = &'a str;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day06.txt", false).collect();

    let groups: Vec<Group> = lines
        .split(|line| line.is_empty())
        .map(|group| parse_group(group))
        .collect();

    let unique: usize = count_unique(&groups);
    let all: usize = count_all(&groups);

    println!("{:?}", unique);
    println!("{:?}", all);
}

fn parse_group(group: &[String]) -> Group {
    group.iter().map(|person| parse_person(person)).collect()
}

fn parse_person(person: &str) -> Person {
    person
        .split("")
        .filter(|answer| !answer.is_empty())
        .collect()
}

fn count_unique(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut answers: HashSet<&str> = HashSet::new();

            for answer in group.iter().flatten() {
                answers.insert(answer);
            }

            answers.len()
        })
        .sum()
}

fn count_all(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut answer_count: HashMap<&str, usize> = HashMap::new();

            for answer in group.iter().flatten() {
                let count = answer_count.entry(answer).or_insert(0);

                *count += 1;
            }

            answer_count
                .into_iter()
                .filter(|(_, count)| *count == group.len())
                .count()
        })
        .sum()
}
