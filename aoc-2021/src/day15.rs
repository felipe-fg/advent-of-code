use aoc_utils;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Cost = usize;
type Grid = Vec<Vec<Cost>>;

type Axis = isize;
type X = Axis;
type Y = Axis;
type Position = (X, Y);

const COST_MAX: Cost = Cost::MAX;
const COST_WRAP: Cost = 9;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    cost: Cost,
}

impl State {
    fn new(position: Position, cost: Cost) -> Self {
        Self { position, cost }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day15.txt", true).collect();

    let grid = parse_grid(lines);

    let expanded_grid = expand_grid(&grid, 5);

    let cost = find_lowest_total_cost(&grid).expect("cost");
    let cost_expanded = find_lowest_total_cost(&expanded_grid).expect("cost expanded");

    println!("{}", cost);
    println!("{}", cost_expanded);
}

fn parse_grid(lines: Vec<String>) -> Grid {
    lines
        .par_iter()
        .map(|line| {
            line.split("")
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .map(|value| value.parse().expect("cost"))
                .collect()
        })
        .collect()
}

fn expand_grid(grid: &Grid, count: usize) -> Grid {
    let rows = grid.len();
    let columns = grid.first().map(|row| row.len()).unwrap_or(0);

    (0..rows * count)
        .map(|row_index| {
            (0..columns * count)
                .map(|column_index| {
                    let original_row = row_index % rows;
                    let original_column = column_index % columns;
                    let original_cost = grid[original_row][original_column];

                    let expanded_row = row_index / rows;
                    let expanded_column = column_index / columns;

                    (original_cost + expanded_row + expanded_column - 1) % COST_WRAP + 1
                })
                .collect()
        })
        .collect()
}

fn find_lowest_total_cost(grid: &Grid) -> Option<Cost> {
    let rows = grid.len();
    let columns = grid.first().map(|row| row.len()).unwrap_or(0);

    let start = (0, 0);
    let goal = (columns as Axis - 1, rows as Axis - 1);

    dijkstra_search(&grid, start, goal)
}

fn dijkstra_search(grid: &Grid, start: Position, goal: Position) -> Option<Cost> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut distance: HashMap<Position, Cost> = HashMap::new();

    heap.push(State::new(start, 0));
    distance.insert(start, 0);

    while let Some(State { position, cost }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        for next in get_neighbors(grid, position) {
            let new_next_cost = cost + get_cost(grid, next);

            let next_cost = distance.get(&next).map(|&cost| cost).unwrap_or(COST_MAX);

            if new_next_cost < next_cost {
                heap.push(State::new(next, new_next_cost));
                distance.insert(next, new_next_cost);
            }
        }
    }

    None
}

fn get_neighbors(grid: &Grid, (current_x, current_y): Position) -> Vec<Position> {
    let rows = grid.len();
    let columns = grid.first().map(|row| row.len()).unwrap_or(0);

    vec![(0, -1), (0, 1), (-1, 0), (1, 0)]
        .into_iter()
        .map(|(direction_x, direction_y)| {
            let x = current_x + direction_x;
            let y = current_y + direction_y;

            (x, y)
        })
        .filter(|&(neighbor_x, neighbor_y)| {
            let valid_x = neighbor_x >= 0 && neighbor_x < columns as Axis;
            let valid_y = neighbor_y >= 0 && neighbor_y < rows as Axis;

            valid_x && valid_y
        })
        .collect()
}

fn get_cost(grid: &Grid, (x, y): Position) -> Cost {
    grid.get(y as usize)
        .map(|row| row.get(x as usize).map(|&cost| cost))
        .flatten()
        .unwrap_or(COST_MAX)
}
