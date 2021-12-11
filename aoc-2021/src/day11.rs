use aoc_utils;
use colored::*;
use rayon::prelude::*;

type Flash = usize;
type Step = usize;
type Energy = usize;
type Grid = Vec<Vec<Energy>>;

const ENERGY_BEFORE_FLASH: Energy = 9;
const ENERGY_FLASH: Energy = 10;
const ENERGY_AFTER_FLASH: Energy = 11;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day11.txt", true).collect();

    let grid = parse_grid(lines);

    let (grid_flash, flash) = find_flash_after_steps(&grid, 100);
    let (grid_step, step) = find_step_flash_all(&grid);

    println!("Flash {}", flash);
    draw_grid(&grid_flash);

    println!();

    println!("Step {}", step);
    draw_grid(&grid_step);
}

fn parse_grid(lines: Vec<String>) -> Grid {
    lines
        .par_iter()
        .map(|line| {
            line.split("")
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .map(|value| value.parse().expect("energy"))
                .collect()
        })
        .collect()
}

fn find_flash_after_steps(grid: &Grid, steps: usize) -> (Grid, Flash) {
    let mut grid = grid.clone();
    let mut flash = 0;

    for _ in 0..steps {
        let (next_grid, next_flash) = simulate_step(&grid);

        grid = next_grid;
        flash += next_flash;
    }

    (grid, flash)
}

fn find_step_flash_all(grid: &Grid) -> (Grid, Step) {
    let rows = grid.len();
    let columns = grid.first().map(|row| row.len()).unwrap_or(0);
    let count = rows * columns;

    let mut grid = grid.clone();

    for step in 1.. {
        let (next_grid, next_flash) = simulate_step(&grid);

        grid = next_grid;

        if next_flash == count {
            return (grid, step);
        }
    }

    (grid, Step::MAX)
}

fn simulate_step(grid: &Grid) -> (Grid, Flash) {
    let mut next_grid = grid.clone();

    increase_energy(&mut next_grid);

    flash_adjacent(&mut next_grid);

    let next_flash = count_flash(&next_grid);

    reset_flash(&mut next_grid);

    (next_grid, next_flash)
}

fn increase_energy(grid: &mut Grid) {
    grid.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|energy| *energy += 1));
}

fn flash_adjacent(grid: &mut Grid) {
    let rows = grid.len();
    let columns = grid.first().map(|row| row.len()).unwrap_or(0);

    loop {
        let mut flash = false;

        for y in 0..rows {
            for x in 0..columns {
                if grid[y][x] == ENERGY_FLASH {
                    grid[y][x] = ENERGY_AFTER_FLASH;
                    flash = true;

                    let min_x = (x as isize - 1).max(0) as usize;
                    let max_x = (x as isize + 1).min(columns as isize - 1) as usize;
                    let min_y = (y as isize - 1).max(0) as usize;
                    let max_y = (y as isize + 1).min(rows as isize - 1) as usize;

                    for adjacent_y in min_y..max_y + 1 {
                        for adjacent_x in min_x..max_x + 1 {
                            let is_adjacent = adjacent_y != y || adjacent_x != x;
                            let is_flash = grid[adjacent_y][adjacent_x] >= ENERGY_FLASH;

                            if is_adjacent && !is_flash {
                                grid[adjacent_y][adjacent_x] += 1;
                            }
                        }
                    }
                }
            }
        }

        if !flash {
            break;
        }
    }
}

fn count_flash(grid: &Grid) -> Flash {
    grid.iter()
        .flat_map(|row| row.iter().filter(|&&energy| energy >= ENERGY_FLASH))
        .count()
}

fn reset_flash(grid: &mut Grid) {
    grid.iter_mut()
        .flat_map(|row| row.iter_mut().filter(|&&mut energy| energy >= ENERGY_FLASH))
        .for_each(|energy| *energy = 0);
}

fn draw_grid(grid: &Grid) {
    for row in grid {
        for &energy in row {
            if energy >= ENERGY_BEFORE_FLASH {
                print!("{}", format!(" {} ", energy).black().on_bright_white());
            } else {
                print!("{}", format!(" {} ", energy).white().on_black());
            }
        }

        println!();
    }
}
