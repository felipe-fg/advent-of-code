use aoc_utils;
use itertools::Itertools;
use std::collections::HashMap;

type Pixel = bool;

type Value = i64;
type X = Value;
type Y = Value;
type Point = (X, Y);

type Grid = HashMap<Point, Pixel>;
type Algorithm = Vec<Pixel>;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day20.txt", true).collect();

    let (algorithm, grid) = parse_enhancement_algorithm(lines);

    let grid_2 = simulate_steps(&algorithm, &grid, 2);
    let grid_50 = simulate_steps(&algorithm, &grid, 50);

    let count_2 = count_pixels(&grid_2);
    let count_50 = count_pixels(&grid_50);

    println!("{}", count_2);
    println!("{}", count_50);
}

fn parse_enhancement_algorithm(lines: Vec<String>) -> (Algorithm, Grid) {
    let algorithm = lines[0].chars().map(|pixel| pixel == '#').collect();

    let grid = lines[1..]
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(move |(column_index, character)| {
                    let point = (column_index as X, row_index as Y);
                    let pixel = character == '#';

                    (point, pixel)
                })
        })
        .collect();

    (algorithm, grid)
}

fn simulate_steps(algorithm: &Algorithm, initial_grid: &Grid, steps: usize) -> Grid {
    let mut grid = initial_grid.clone();

    let mut min_x = grid.keys().map(|&(x, _)| x).min().expect("min x") - 1;
    let mut max_x = grid.keys().map(|&(x, _)| x).max().expect("max x") + 1;
    let mut min_y = grid.keys().map(|&(_, y)| y).min().expect("min y") - 1;
    let mut max_y = grid.keys().map(|&(_, y)| y).max().expect("max y") + 1;

    let first = algorithm.first().expect("first").clone();
    let last = algorithm.last().expect("last").clone();

    let mut border = false;

    for _ in 0..steps {
        grid = next_state(&algorithm, &grid, border, (min_x, max_x, min_y, max_y));

        min_x -= 1;
        max_x += 1;
        min_y -= 1;
        max_y += 1;

        if !border && first {
            border = first;
        } else if border && !last {
            border = last;
        }
    }

    grid
}

fn next_state(
    algorithm: &Algorithm,
    grid: &Grid,
    border: Pixel,
    (min_x, max_x, min_y, max_y): (X, X, Y, Y),
) -> Grid {
    (min_x..=max_x)
        .into_iter()
        .flat_map(|x| {
            (min_y..=max_y).into_iter().map(move |y| {
                let point = (x, y);
                let pixel = get_algorithm_pixel(algorithm, grid, point, border);

                (point, pixel)
            })
        })
        .collect()
}

fn get_algorithm_pixel(algorithm: &Algorithm, grid: &Grid, (x, y): Point, border: Pixel) -> Pixel {
    let binary = (y - 1..=y + 1)
        .flat_map(|y| {
            (x - 1..=x + 1).map(move |x| {
                let point = (x, y);
                let pixel = get_grid_pixel(grid, point, border);

                match pixel {
                    true => 1,
                    false => 0,
                }
            })
        })
        .join("");

    let index = to_decimal(&binary);

    algorithm[index]
}

fn get_grid_pixel(grid: &Grid, point: Point, border: Pixel) -> Pixel {
    grid.get(&point).map(|&pixel| pixel).unwrap_or(border)
}

fn to_decimal(binary: &str) -> usize {
    usize::from_str_radix(binary, 2).expect("decimal")
}

fn count_pixels(grid: &Grid) -> usize {
    grid.values().filter(|&&pixel| pixel).count()
}
