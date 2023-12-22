use std::collections::HashSet;

use colored::*;

type Number = i64;
type Position = (i64, i64);
type Positions = HashSet<Position>;

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Start,
    Plot,
    Rock,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day21.txt", true).collect();

    let map = parse_map(&lines);

    let plots_64 = find_reachable_plots(&map, 64);

    draw_map(&map, &plots_64, 1);
    println!("{:?}", plots_64.len());

    let plots_26501365 = count_reachable_plots_lagrange_polynomial(&map, 26501365);

    println!("{:?}", plots_26501365);
}

fn parse_map(lines: &[String]) -> Map {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::Plot,
                    'S' => Tile::Start,
                    '#' => Tile::Rock,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn find_reachable_plots(map: &Map, steps: u64) -> Positions {
    let mut current_positions = Positions::new();

    current_positions.insert(find_start_position(map));

    for _ in 0..steps {
        let next_positions = current_positions
            .iter()
            .flat_map(|current_position| get_neighbors(map, current_position))
            .collect();

        current_positions = next_positions;
    }

    current_positions
}

fn count_reachable_plots_lagrange_polynomial(map: &Map, x: u64) -> u64 {
    let size_x = map.first().map(|row| row.len()).unwrap_or_default() as f64;

    let x = x as f64;
    let x0 = 65.0;
    let x1 = x0 + size_x;
    let x2 = x0 + size_x * 2.0;

    let y0 = find_reachable_plots(map, x0 as u64).len() as f64;
    let y1 = find_reachable_plots(map, x1 as u64).len() as f64;
    let y2 = find_reachable_plots(map, x2 as u64).len() as f64;

    let l0 = ((x - x1) / (x0 - x1)) * ((x - x2) / (x0 - x2));
    let l1 = ((x - x0) / (x1 - x0)) * ((x - x2) / (x1 - x2));
    let l2 = ((x - x0) / (x2 - x0)) * ((x - x1) / (x2 - x1));

    (y0 * l0 + y1 * l1 + y2 * l2) as u64
}

fn get_neighbors(map: &Map, &(position_x, position_y): &Position) -> Positions {
    vec![(0, -1), (0, 1), (-1, 0), (1, 0)]
        .into_iter()
        .filter_map(|(direction_x, direction_y)| {
            let next_position = (position_x + direction_x, position_y + direction_y);

            get_tile(map, &next_position)
                .filter(|tile| tile != &Tile::Rock)
                .map(move |_| next_position)
        })
        .collect()
}

fn find_start_position(map: &Map) -> Position {
    map.iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .find(|(_, tile)| tile == &&Tile::Start)
                .map(|(column_index, _)| (column_index as Number, row_index as Number))
        })
        .expect("start")
}

fn get_tile(map: &Map, position: &Position) -> Option<Tile> {
    let (x, y) = get_local_position(map, position);

    map.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .cloned()
}

fn get_local_position(map: &Map, &(global_x, global_y): &Position) -> Position {
    let size_x = map.first().map(|row| row.len()).unwrap_or_default() as Number;
    let size_y = map.len() as Number;

    let local_x = if global_x >= 0 {
        global_x % size_x
    } else {
        ((global_x % size_x) + size_x) % size_x
    };

    let local_y = if global_y >= 0 {
        global_y % size_y
    } else {
        ((global_y % size_y) + size_y) % size_y
    };

    (local_x, local_y)
}

fn draw_map(map: &Map, plots: &Positions, scale: Number) {
    let size_x = map.first().map(|row| row.len()).unwrap_or_default() as Number;
    let size_y = map.len() as Number;

    let start_x = -((scale - 1) * (size_x));
    let end_x = size_x + ((scale - 1) * size_x);

    let start_y = -((scale - 1) * (size_y));
    let end_y = size_y + ((scale - 1) * size_y);

    for row_index in start_y..end_y {
        for column_index in start_x..end_x {
            let position = (column_index as Number, row_index as Number);

            let tile = get_tile(map, &position).expect("tile");
            let is_plot = plots.contains(&position);

            let tile = match tile {
                Tile::Start if is_plot => "█".bright_red(),
                Tile::Plot if is_plot => "█".red(),
                Tile::Start => "█".bright_green(),
                Tile::Plot => "█".green(),
                Tile::Rock => "█".bright_black(),
            };

            print!("{}", tile);
        }

        println!();
    }
}
