use super::utils;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type Color = String;
type Count = usize;
type RuleMap = HashMap<Color, RestrictionMap>;
type RestrictionMap = HashMap<Color, Count>;

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day07.txt", true).collect();

    let rules = parse_rules(lines);

    let shiny_gold_outwards = count_outwards(&rules, "shiny gold");
    let shiny_gold_inward = count_inward(&rules, "shiny gold") - 1;

    println!("{:?}", shiny_gold_outwards);
    println!("{:?}", shiny_gold_inward);
}

fn parse_rules(lines: Vec<String>) -> RuleMap {
    lines.par_iter().map(|line| parse_rule(line)).collect()
}

fn parse_rule(line: &str) -> (Color, RestrictionMap) {
    let mut parts = line.split("bags contain");

    let rule_color = parts.next().expect("rule_color").trim().to_string();

    let rule_restrictions: RestrictionMap = parts
        .next()
        .expect("rule_restrictions")
        .trim()
        .split(",")
        .filter(|restriction| !restriction.contains("no other"))
        .map(|restriction| parse_restriction(restriction))
        .collect();

    (rule_color, rule_restrictions)
}

fn parse_restriction(restriction: &str) -> (Color, Count) {
    let caps = Regex::new(r"(?P<count>\d+)\s(?P<color>.+)\sbag")
        .expect("regex")
        .captures(restriction)
        .expect("captures");

    let color = caps["color"].to_string();
    let count: Count = caps["count"].to_string().parse().expect("count");

    (color, count)
}

fn count_outwards(rules: &RuleMap, bag_color: &str) -> usize {
    let mut all_colors: HashSet<&str> = HashSet::new();
    let mut current_colors = vec![bag_color];

    while !current_colors.is_empty() {
        let next_colors: Vec<&str> = rules
            .iter()
            .filter(|(_, restrictions)| {
                current_colors
                    .iter()
                    .any(|bag_color| restrictions.contains_key::<str>(bag_color))
            })
            .map(|(color, _)| color.as_ref())
            .collect();

        for next_color in &next_colors {
            all_colors.insert(next_color);
        }

        current_colors = next_colors;
    }

    all_colors.len()
}

fn count_inward(rules: &RuleMap, bag_color: &str) -> usize {
    let restrictions = rules.get(bag_color).expect("restrictions");

    let count: usize = restrictions
        .iter()
        .map(|(color, count)| count * count_inward(rules, color))
        .sum();

    1 + count
}
