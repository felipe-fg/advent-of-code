use aoc_utils;
use std::iter;

type Ticket = Vec<usize>;

#[derive(Debug)]
struct Document {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(Debug, PartialEq)]
struct Rule {
    id: String,
    ranges: Vec<(usize, usize)>,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day16.txt", true).collect();

    let document = parse_document(&lines);

    let error_rate = ticket_scanning_error_rate(&document);

    let field_order = build_field_order(&document);
    let departure = departure_total(&document, &field_order);

    println!("{:?}", error_rate);
    println!("{:?}", departure);
}

fn parse_document(lines: &Vec<String>) -> Document {
    let mut rules = vec![];
    let mut your_ticket = vec![];
    let mut nearby_tickets = vec![];

    let mut section = "rules";

    for line in lines {
        match (&line[..], &section[..]) {
            ("your ticket:", _) => section = "your_ticket",
            ("nearby tickets:", _) => section = "nearby_tickets",

            (_, "rules") => {
                let mut parts = line.split(":");

                let id = parts.next().expect("id").to_string();

                let ranges = parts
                    .next()
                    .expect("ranges")
                    .split("or")
                    .map(|range| {
                        let mut values = range
                            .trim()
                            .split("-")
                            .map(|value| value.parse::<usize>().expect("value"));

                        let min = values.next().expect("min");
                        let max = values.next().expect("max");

                        (min, max)
                    })
                    .collect();

                let rule = Rule {
                    id: id,
                    ranges: ranges,
                };

                rules.push(rule);
            }
            (_, "your_ticket") => {
                your_ticket = line
                    .split(",")
                    .map(|value| value.parse().expect("value"))
                    .collect();
            }
            (_, "nearby_tickets") => {
                nearby_tickets.push(
                    line.split(",")
                        .map(|value| value.parse().expect("value"))
                        .collect(),
                );
            }
            _ => (),
        }
    }

    Document {
        rules: rules,
        your_ticket: your_ticket,
        nearby_tickets: nearby_tickets,
    }
}

fn ticket_scanning_error_rate(document: &Document) -> usize {
    document
        .nearby_tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .iter()
                .filter(|value| !is_value_valid(&document.rules, value))
                .map(|value| *value)
        })
        .sum()
}

fn build_field_order(document: &Document) -> Vec<(String, usize)> {
    let tickets: Vec<&Ticket> = iter::once(&document.your_ticket)
        .chain(document.nearby_tickets.iter())
        .filter(|ticket| is_ticket_valid(document, ticket))
        .collect();

    let mut field_order = vec![];
    let mut remaining_rules: Vec<&Rule> = document.rules.iter().collect();
    let mut remaining_positions: Vec<usize> = (0..document.rules.len()).collect();

    while !remaining_rules.is_empty() {
        let (rule, positions) = remaining_rules
            .iter()
            .map(|rule| {
                let positions: Vec<usize> = remaining_positions
                    .iter()
                    .filter(|position| {
                        tickets
                            .iter()
                            .all(|ticket| is_rule_value_valid(rule, &ticket[**position]))
                    })
                    .map(|position| *position)
                    .collect();

                (rule, positions)
            })
            .find(|(_, positions)| positions.len() == 1)
            .expect("rule positions");

        let id = rule.id.to_string();
        let position = positions[0];

        remaining_rules.retain(|x| x.id != id);
        remaining_positions.retain(|x| x != &position);

        field_order.push((id, position));
    }

    field_order
}

fn departure_total(document: &Document, field_order: &Vec<(String, usize)>) -> usize {
    field_order
        .iter()
        .filter(|(id, _)| id.contains("departure"))
        .map(|(_, position)| document.your_ticket[*position])
        .fold(1, |acc, x| acc * x)
}

fn is_ticket_valid(document: &Document, ticket: &Ticket) -> bool {
    ticket
        .iter()
        .all(|value| is_value_valid(&document.rules, value))
}

fn is_value_valid(rules: &Vec<Rule>, value: &usize) -> bool {
    rules.iter().any(|rule| is_rule_value_valid(rule, value))
}

fn is_rule_value_valid(rule: &Rule, value: &usize) -> bool {
    rule.ranges
        .iter()
        .any(|(min, max)| value >= min && value <= max)
}
