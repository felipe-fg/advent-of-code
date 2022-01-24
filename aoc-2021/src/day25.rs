use aoc_utils;
use colored::*;

type Seafloor = Vec<Vec<char>>;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day25.txt", true).collect();

    let initial_seafloor = parse_seafloor(lines);

    let (seafloor, step) = compute_seafloor(&initial_seafloor);

    draw_seafloor(&seafloor, step);
}

fn parse_seafloor(lines: Vec<String>) -> Seafloor {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn compute_seafloor(initial: &Seafloor) -> (Seafloor, usize) {
    let mut current = initial.clone();

    for step in 1.. {
        let next = step_seafloor(&current);

        if next == current {
            return (next, step);
        } else {
            current = next;
        }
    }

    panic!()
}

fn step_seafloor(seafloor: &Seafloor) -> Seafloor {
    let seafloor = step_seafloor_east(seafloor);
    let seafloor = step_seafloor_south(&seafloor);

    seafloor
}

fn step_seafloor_east(seafloor: &Seafloor) -> Seafloor {
    let columns = seafloor.first().map(|row| row.len()).unwrap_or(0);

    seafloor
        .iter()
        .enumerate()
        .map(|(index_row, tiles)| {
            tiles
                .iter()
                .enumerate()
                .map(|(index_column, &current)| {
                    let previous_column = (index_column + columns - 1) % columns;
                    let next_column = (index_column + columns + 1) % columns;

                    let previous = seafloor[index_row][previous_column];
                    let next = seafloor[index_row][next_column];

                    if current == '.' && previous == '>' {
                        '>'
                    } else if current == '>' && next == '.' {
                        '.'
                    } else {
                        current
                    }
                })
                .collect()
        })
        .collect()
}

fn step_seafloor_south(seafloor: &Seafloor) -> Seafloor {
    let rows = seafloor.len();

    seafloor
        .iter()
        .enumerate()
        .map(|(index_row, tiles)| {
            tiles
                .iter()
                .enumerate()
                .map(|(index_column, &current)| {
                    let previous_row = (index_row + rows - 1) % rows;
                    let next_row = (index_row + rows + 1) % rows;

                    let previous = seafloor[previous_row][index_column];
                    let next = seafloor[next_row][index_column];

                    if current == '.' && previous == 'v' {
                        'v'
                    } else if current == 'v' && next == '.' {
                        '.'
                    } else {
                        current
                    }
                })
                .collect()
        })
        .collect()
}

fn draw_seafloor(seafloor: &Seafloor, step: usize) {
    println!("Step {}", step);

    for rows in seafloor {
        for &tile in rows {
            if tile == '>' {
                print!("{}", ">".bright_red().on_black());
            } else if tile == 'v' {
                print!("{}", "v".bright_green().on_black());
            } else {
                print!("{}", ".".bright_blue().on_black());
            }
        }

        println!();
    }
}
