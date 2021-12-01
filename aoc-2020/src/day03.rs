use aoc_utils;

const TREE: char = '#';

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day03.txt", true).collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let counts: Vec<usize> = slopes
        .into_iter()
        .map(|slope| count_trees(&lines, slope))
        .collect();

    let count_slope = counts.get(1).expect("count");
    let count_all: usize = counts.iter().product();

    println!("{}", count_slope);
    println!("{}", count_all);
}

fn count_trees(map: &Vec<String>, (slope_right, slope_down): (usize, usize)) -> usize {
    map.iter()
        .step_by(slope_down)
        .enumerate()
        .filter(|(i, row)| is_tree(row, i * slope_right))
        .count()
}

fn is_tree(row: &str, right: usize) -> bool {
    row.chars().cycle().nth(right).expect("position") == TREE
}
