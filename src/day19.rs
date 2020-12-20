use super::utils;
use itertools::join;
use regex::Regex;
use std::collections::HashMap;

type Rules = HashMap<String, Vec<Vec<String>>>;

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day19.txt", false).collect();

    let (lines_rules, lines_messages) = parse_input(&lines);
    let mut rules = parse_rules(lines_rules);

    let regex_one = build_regex(&rules, "0");
    let count_one = count_messages(lines_messages, &regex_one);

    update_rules_loop(&mut rules);

    let regex_two = build_regex(&rules, "0");
    let count_two = count_messages(lines_messages, &regex_two);

    println!("{}", count_one);
    println!("{}", count_two);
}

fn parse_input(lines: &Vec<String>) -> (&[String], &[String]) {
    let empty_index = lines
        .iter()
        .position(|line| line.is_empty())
        .expect("empty index");

    let lines_rules = &lines[..empty_index];
    let lines_messages = &lines[empty_index + 1..lines.len()];

    (lines_rules, lines_messages)
}

fn parse_rules(lines: &[String]) -> Rules {
    lines
        .iter()
        .map(|line| {
            let mut iter = line.split(": ");

            let id = iter.next().expect("id").to_string();

            let sub_rules = iter
                .next()
                .expect("sub_rules")
                .replace("\"", "")
                .split(" | ")
                .map(|sub_rule| sub_rule.split(" ").map(|item| item.to_string()).collect())
                .collect();

            (id, sub_rules)
        })
        .collect()
}

fn build_regex(rules: &Rules, id: &str) -> String {
    fn build(rules: &Rules, id: &str, loop_max: usize, loop_index: usize) -> String {
        let sub_rules = &rules[id];

        let character_item = sub_rules
            .first()
            .map(|items| items.first())
            .flatten()
            .filter(|item| item == &"a" || item == &"b");

        if let Some(character) = character_item {
            format!("{}", character)
        } else {
            let iter = sub_rules.iter().map(|items| {
                let iter = items.iter().map(|item| {
                    let is_loop = item == id;

                    if is_loop {
                        if loop_index < loop_max {
                            build(rules, item, loop_max, loop_index + 1)
                        } else {
                            String::from("")
                        }
                    } else {
                        build(rules, item, loop_max, 0)
                    }
                });

                join(iter, "")
            });

            format!("(?:{})", join(iter, "|"))
        }
    }

    format!("\\b{}\\b", build(rules, id, 9, 0))
}

fn count_messages(messages: &[String], regex: &str) -> usize {
    let re = Regex::new(&regex).expect("regex");

    messages
        .iter()
        .filter(|message| re.is_match(message))
        .count()
}

fn update_rules_loop(rules: &mut Rules) {
    rules.entry(String::from("8")).and_modify(|sub_rules| {
        *sub_rules = vec![
            vec![String::from("42")],
            vec![String::from("42"), String::from("8")],
        ]
    });

    rules.entry(String::from("11")).and_modify(|sub_rules| {
        *sub_rules = vec![
            vec![String::from("42"), String::from("31")],
            vec![String::from("42"), String::from("11"), String::from("31")],
        ]
    });
}
