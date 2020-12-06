use super::utils;
use rayon::prelude::*;
use regex::Regex;

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day04.txt", false).collect();

    let passports = lines
        .split(|line| line.is_empty())
        .map(|lines| extract_fields(lines));

    let required_valid: Vec<Vec<&str>> = passports
        .filter(|fields| is_required_rule_valid(fields))
        .collect();

    let value_valid = required_valid
        .par_iter()
        .filter(|fields| is_value_rule_valid(fields))
        .count();

    println!("{}", required_valid.len());
    println!("{}", value_valid);
}

fn extract_fields(lines: &[String]) -> Vec<&str> {
    lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect()
}

fn is_required_rule_valid(fields: &[&str]) -> bool {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let found = fields
        .iter()
        .filter(|field| required.iter().any(|required| field.contains(required)))
        .count();

    found >= required.len()
}

fn is_value_rule_valid(fields: &[&str]) -> bool {
    let valid_values = fields.iter().filter(|field| is_field_valid(field)).count();

    valid_values == fields.len()
}

fn is_field_valid(field: &str) -> bool {
    let pair: Vec<&str> = field.split(":").collect();

    match &pair[..] {
        ["byr", value] => is_birth_year_valid(value),
        ["iyr", value] => is_issue_year_valid(value),
        ["eyr", value] => is_expiration_year_valid(value),
        ["hgt", value] => is_height_valid(value),
        ["hcl", value] => is_hair_color_valid(value),
        ["ecl", value] => is_eye_color_valid(value),
        ["pid", value] => is_passport_id_valid(value),
        _ => true,
    }
}

fn is_birth_year_valid(value: &str) -> bool {
    value
        .parse::<usize>()
        .map(|number| number >= 1920 && number <= 2002)
        .unwrap_or(false)
}

fn is_issue_year_valid(value: &str) -> bool {
    value
        .parse::<usize>()
        .map(|number| number >= 2010 && number <= 2020)
        .unwrap_or(false)
}

fn is_expiration_year_valid(value: &str) -> bool {
    value
        .parse::<usize>()
        .map(|number| number >= 2020 && number <= 2030)
        .unwrap_or(false)
}

fn is_height_valid(value: &str) -> bool {
    Regex::new(r"(?P<number>\d+)(?P<unit>cm|in)")
        .expect("regex")
        .captures(value)
        .map(|caps| {
            let number: usize = caps["number"].to_string().parse().expect("number");
            let unit = caps["unit"].to_string();

            match &unit[..] {
                "cm" => number >= 150 && number <= 193,
                "in" => number >= 59 && number <= 76,
                _ => false,
            }
        })
        .unwrap_or(false)
}

fn is_hair_color_valid(value: &str) -> bool {
    Regex::new(r"#[0-9a-f]{6}").expect("regex").is_match(value)
}

fn is_eye_color_valid(value: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
}

fn is_passport_id_valid(value: &str) -> bool {
    Regex::new(r"\b\d{9}\b").expect("regex").is_match(value)
}
