use std::collections::HashMap;
use std::iter;

type Platform = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Sphere,
    Cube,
    Empty,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day14.txt", true).collect();

    let platform = parse_platform(&lines);

    let tilted_top = tilt_vertical(&platform, true);
    let load_top = compute_platform_load(&tilted_top);

    let tilted_cycle = find_platform_at_cycle(&platform, 1000000000);
    let load_cycle = compute_platform_load(&tilted_cycle);

    println!("{}", load_top);
    println!("{}", load_cycle);
}

fn parse_platform(lines: &[String]) -> Platform {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'O' => Tile::Sphere,
                    '#' => Tile::Cube,
                    '.' => Tile::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn find_platform_at_cycle(initial_platform: &Platform, cycles: usize) -> Platform {
    let loop_offset;
    let loop_length;

    let mut previous_states = HashMap::new();
    let mut current_platform = initial_platform.clone();
    let mut current_cycle = 0;

    loop {
        if let Some(previous_cycle) = previous_states.get(&current_platform) {
            loop_offset = *previous_cycle;
            loop_length = current_cycle - previous_cycle;
            break;
        }

        previous_states.insert(current_platform.clone(), current_cycle);

        let next_platform = execute_spin_cycle(current_platform);

        current_cycle += 1;
        current_platform = next_platform;
    }

    let final_cycle = ((cycles - loop_offset) % loop_length) + loop_offset;

    previous_states
        .iter()
        .find(|(_, cycle)| cycle == &&final_cycle)
        .map(|(platform, _)| platform.clone())
        .expect("platform")
}

fn execute_spin_cycle(platform: Platform) -> Platform {
    let north = tilt_vertical(&platform, true);
    let west = tilt_horizontal(&north, true);
    let south = tilt_vertical(&west, false);
    tilt_horizontal(&south, false)
}

fn tilt_vertical(platform: &Platform, top: bool) -> Platform {
    let transposed = transpose_platform(platform);

    let tilted = tilt_horizontal(&transposed, top);

    transpose_platform(&tilted)
}

fn tilt_horizontal(platform: &Platform, left: bool) -> Platform {
    platform
        .iter()
        .map(|row| tilt_horizontal_row(row, left))
        .collect()
}

fn tilt_horizontal_row(initial_row: &[Tile], left: bool) -> Vec<Tile> {
    let mut current_row = initial_row.to_vec();

    loop {
        let next_row: Vec<_> = get_iterator_row(&current_row)
            .map(
                |(previous, current, next)| match (previous, current, next) {
                    (Some(Tile::Empty), Tile::Sphere, _) if left => Tile::Empty,
                    (_, Tile::Empty, Some(Tile::Sphere)) if left => Tile::Sphere,
                    (Some(Tile::Sphere), Tile::Empty, _) if !left => Tile::Sphere,
                    (_, Tile::Sphere, Some(Tile::Empty)) if !left => Tile::Empty,
                    (_, current, _) => *current,
                },
            )
            .collect();

        if next_row == current_row {
            break;
        } else {
            current_row = next_row;
        }
    }

    current_row
}

fn compute_platform_load(platform: &Platform) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(index, row)| {
            let count = row.iter().filter(|tile| tile == &&Tile::Sphere).count();

            count * (index + 1)
        })
        .sum()
}

fn get_iterator_row(row: &[Tile]) -> impl Iterator<Item = (Option<&Tile>, &Tile, Option<&Tile>)> {
    let mut index = 0;

    iter::from_fn(move || {
        let previous = if index > 0 { row.get(index - 1) } else { None };
        let current = row.get(index);
        let next = row.get(index + 1);

        index += 1;

        current.map(|current| (previous, current, next))
    })
}

fn transpose_platform(platform: &Platform) -> Platform {
    let rows = platform.len();
    let columns = platform.first().map(|row| row.len()).unwrap_or_default();

    (0..columns)
        .map(|column| (0..rows).map(|row| platform[row][column]).collect())
        .collect()
}
