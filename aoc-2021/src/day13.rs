use aoc_utils;
use colored::*;
use itertools::Itertools;
use regex::Regex;

type X = isize;
type Y = isize;
type Dot = (X, Y);

type Axis = String;
type Value = isize;
type Fold = (Axis, Value);

#[derive(Debug, Clone)]
struct Manual {
    dots: Vec<Dot>,
    folds: Vec<Fold>,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day13.txt", true).collect();

    let manual = parse_manual(lines);

    let fold_count = apply_folds(&manual, 1);
    let fold_all = apply_folds(&manual, usize::MAX);

    println!("{}", fold_count.len());
    draw_paper(&fold_all);
}

fn parse_manual(lines: Vec<String>) -> Manual {
    let re_dot = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").expect("regex dot");
    let re_fold = Regex::new(r"fold along (?P<axis>[xy])=(?P<value>\d+)").expect("regex fold");

    let mut dots = vec![];
    let mut folds = vec![];

    for line in lines {
        if line.contains("fold") {
            let caps = re_fold.captures(&line).expect("captures");
            let axis = caps["axis"].trim().to_string().to_lowercase();
            let value = caps["value"].trim().to_string().parse().expect("number");

            folds.push((axis, value));
        } else {
            let caps = re_dot.captures(&line).expect("captures");
            let x = caps["x"].trim().to_string().parse().expect("number");
            let y = caps["y"].trim().to_string().parse().expect("number");

            dots.push((x, y));
        }
    }

    Manual { dots, folds }
}

fn apply_folds(manual: &Manual, count: usize) -> Vec<Dot> {
    manual
        .folds
        .iter()
        .take(count)
        .fold(manual.dots.clone(), |dots, fold| fold_paper(&dots, fold))
}

fn fold_paper(dots: &[Dot], fold: &Fold) -> Vec<Dot> {
    dots.iter()
        .map(|dot| fold_dot(dot, fold))
        .unique()
        .collect()
}

fn fold_dot(&(x, y): &Dot, (axis, value): &Fold) -> Dot {
    match &axis[..] {
        "y" if &y >= value => (x, value - (y - value)),
        "x" if &x >= value => (value - (x - value), y),
        _ => (x, y),
    }
}

fn draw_paper(dots: &[Dot]) {
    let min_x: X = dots.iter().map(|(x, _)| x).min().expect("min").to_owned();
    let max_x: X = dots.iter().map(|(x, _)| x).max().expect("max").to_owned();
    let min_y: Y = dots.iter().map(|(_, y)| y).min().expect("min").to_owned();
    let max_y: Y = dots.iter().map(|(_, y)| y).max().expect("max").to_owned();

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let dot = dots.contains(&(x, y));

            if dot {
                print!("{}", "██".bright_white());
            } else {
                print!("{}", "██".black());
            }
        }

        println!();
    }
}
