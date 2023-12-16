use std::collections::HashSet;

use colored::*;
use rayon::prelude::*;

type Number = i64;
type Position = (Number, Number);
type Positions = HashSet<Position>;
type Ray = (Position, Direction);
type Rays = HashSet<Ray>;
type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Tile {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterVertical,
    SplitterHorizontal,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day16.txt", true).collect();

    let grid = parse_grid(&lines);

    let traces = trace_rays(&grid, ((-1, 0), Direction::Right));
    let traces_best = compute_best_configuration(&grid);

    draw_grid(&grid, &traces);
    println!();
    draw_grid(&grid, &traces_best);

    println!("{}", traces.len());
    println!("{}", traces_best.len());
}

fn trace_rays(grid: &Grid, initial_ray: Ray) -> Positions {
    let mut traced_rays = Rays::new();
    let mut current_rays = Rays::new();

    current_rays.insert(initial_ray);

    while !current_rays.is_empty() {
        let next_rays: Rays = current_rays
            .par_iter()
            .flat_map(|ray| step_ray(grid, ray))
            .collect();

        traced_rays.extend(current_rays);

        current_rays = next_rays
            .into_par_iter()
            .filter(|next_ray| !traced_rays.contains(next_ray))
            .collect();
    }

    traced_rays
        .into_par_iter()
        .filter(|traced_ray| traced_ray != &initial_ray)
        .map(|(position, _)| position)
        .collect()
}

fn step_ray(grid: &Grid, &((ray_position_x, ray_position_y), ray_direction): &Ray) -> Rays {
    let next_position = match ray_direction {
        Direction::Up => (ray_position_x, ray_position_y - 1),
        Direction::Down => (ray_position_x, ray_position_y + 1),
        Direction::Left => (ray_position_x - 1, ray_position_y),
        Direction::Right => (ray_position_x + 1, ray_position_y),
    };

    let next_tile = get_tile(grid, &next_position);

    let next_rays = match next_tile {
        None => vec![],
        Some(Tile::Empty) => vec![(next_position, ray_direction)],
        Some(Tile::MirrorForward) => match ray_direction {
            Direction::Right => vec![(next_position, Direction::Up)],
            Direction::Left => vec![(next_position, Direction::Down)],
            Direction::Down => vec![(next_position, Direction::Left)],
            Direction::Up => vec![(next_position, Direction::Right)],
        },
        Some(Tile::MirrorBackward) => match ray_direction {
            Direction::Right => vec![(next_position, Direction::Down)],
            Direction::Left => vec![(next_position, Direction::Up)],
            Direction::Down => vec![(next_position, Direction::Right)],
            Direction::Up => vec![(next_position, Direction::Left)],
        },
        Some(Tile::SplitterVertical) => match ray_direction {
            Direction::Up | Direction::Down => vec![(next_position, ray_direction)],
            Direction::Left | Direction::Right => {
                vec![
                    (next_position, Direction::Up),
                    (next_position, Direction::Down),
                ]
            }
        },
        Some(Tile::SplitterHorizontal) => match ray_direction {
            Direction::Left | Direction::Right => vec![(next_position, ray_direction)],
            Direction::Up | Direction::Down => {
                vec![
                    (next_position, Direction::Left),
                    (next_position, Direction::Right),
                ]
            }
        },
    };

    Rays::from_iter(next_rays)
}

fn compute_best_configuration(grid: &Grid) -> Positions {
    let rows = grid.len() as Number;
    let columns = grid.first().map(|row| row.len()).unwrap_or_default() as Number;

    let down = (0..columns).map(|index| ((index, -1), Direction::Down));
    let up = (0..columns).map(|index| ((index, rows), Direction::Up));
    let right = (0..rows).map(|index| ((-1, index), Direction::Right));
    let left = (0..rows).map(|index| ((columns, index), Direction::Left));

    let rays: Vec<_> = down.chain(up).chain(right).chain(left).collect();

    rays.into_par_iter()
        .map(|ray| trace_rays(grid, ray))
        .max_by_key(|traces| traces.len())
        .expect("configuration")
}

fn get_tile(grid: &Grid, &(x, y): &Position) -> Option<Tile> {
    if x < 0 || y < 0 {
        None
    } else {
        grid.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .cloned()
    }
}

fn parse_grid(lines: &[String]) -> Grid {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::Empty,
                    '/' => Tile::MirrorForward,
                    '\\' => Tile::MirrorBackward,
                    '-' => Tile::SplitterHorizontal,
                    '|' => Tile::SplitterVertical,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn draw_grid(grid: &Grid, traces: &Positions) {
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, tile) in row.iter().enumerate() {
            let position = (column_index as Number, row_index as Number);
            let is_traced = traces.contains(&position);

            let tile = match tile {
                Tile::Empty if !is_traced => " ".black(),
                Tile::Empty if is_traced => ".".red(),
                Tile::MirrorForward => "/".blue(),
                Tile::MirrorBackward => "\\".blue(),
                Tile::SplitterVertical => "|".bright_blue(),
                Tile::SplitterHorizontal => "─".bright_blue(),
                _ => unreachable!(),
            };

            let tile = if is_traced { tile.red() } else { tile };

            print!("{}", tile.on_black());
        }

        println!();
    }
}
