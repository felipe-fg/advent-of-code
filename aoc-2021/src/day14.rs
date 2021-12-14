use aoc_utils;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type Element = String;
type Polymer = String;
type Count = u128;

type ElementPair = (Element, Element);
type ElementCount = (Element, Count);

type RuleMap = HashMap<ElementPair, Element>;
type PairMap = HashMap<ElementPair, Count>;
type ElementMap = HashMap<Element, Count>;

#[derive(Debug, Clone)]
struct Manual {
    template: Polymer,
    rules: RuleMap,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day14.txt", true).collect();

    let manual = parse_manual(lines);

    let pairs_10 = find_formula_pairs(&manual, 10);
    let pairs_40 = find_formula_pairs(&manual, 40);

    let ((_, most_10), (_, least_10)) = find_most_least_common(&manual, &pairs_10);
    let ((_, most_40), (_, least_40)) = find_most_least_common(&manual, &pairs_40);

    println!("{}", most_10 - least_10);
    println!("{}", most_40 - least_40);
}

fn parse_manual(lines: Vec<String>) -> Manual {
    let re = Regex::new(r"(?P<left>\w)(?P<right>\w) -> (?P<element>\w)").expect("regex");

    let template = lines.first().expect("template").trim().to_uppercase();

    let rules = lines
        .iter()
        .skip(1)
        .map(|line| {
            let caps = re.captures(&line).expect("captures");

            let left = caps["left"].trim().to_uppercase();
            let right = caps["right"].trim().to_uppercase();
            let element = caps["element"].trim().to_uppercase();

            ((left, right), element)
        })
        .collect();

    Manual { template, rules }
}

fn find_formula_pairs(manual: &Manual, steps: usize) -> PairMap {
    let mut pairs = get_pair_map(&manual.template);

    for _ in 0..steps {
        for ((left, right), count) in pairs.clone() {
            let pair = (left.clone(), right.clone());

            if let Some(element) = manual.rules.get(&pair) {
                let left_pair = (left.clone(), element.clone());
                let right_pair = (element.clone(), right.clone());

                *pairs.entry(pair).or_insert(0) -= count;
                *pairs.entry(left_pair).or_insert(0) += count;
                *pairs.entry(right_pair).or_insert(0) += count;
            }
        }
    }

    pairs
}

fn find_most_least_common(manual: &Manual, pairs: &PairMap) -> (ElementCount, ElementCount) {
    let elements = get_element_map(&manual.template, pairs);

    let most = elements
        .iter()
        .max_by_key(|&(_, &count)| count)
        .map(|(element, &count)| (element.clone(), count))
        .expect("most");

    let least = elements
        .iter()
        .min_by_key(|&(_, &count)| count)
        .map(|(element, &count)| (element.clone(), count))
        .expect("least");

    (most, least)
}

fn get_pair_map(polymer: &Polymer) -> PairMap {
    let mut pairs = PairMap::new();

    for (left, right) in polymer.chars().tuple_windows() {
        let pair = (left.to_string(), right.to_string());

        *pairs.entry(pair).or_insert(0) += 1;
    }

    pairs
}

fn get_element_map(polymer: &Polymer, pairs: &PairMap) -> ElementMap {
    let mut elements = ElementMap::new();

    for ((left, _), count) in pairs {
        *elements.entry(left.clone()).or_insert(0) += count;
    }

    let last = polymer.chars().last().expect("last").to_string();

    *elements.entry(last).or_insert(0) += 1;

    elements
}
