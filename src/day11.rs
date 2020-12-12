use super::utils;
use rayon::prelude::*;

type Map = Vec<Vec<State>>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    Floor,
    Empty,
    Occupied,
}

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day11.txt", true).collect();

    let map = parse_map(&lines);

    let stable_immediately = stabilize_map(map.clone(), 4, true);
    let count_immediately = count_map(&stable_immediately, State::Occupied);

    let stable_directions = stabilize_map(map, 5, false);
    let count_directions = count_map(&stable_directions, State::Occupied);

    println!("{:?}", count_immediately);
    println!("{:?}", count_directions);
}

fn parse_map(lines: &Vec<String>) -> Map {
    lines
        .par_iter()
        .map(|line| {
            line.chars()
                .map(|position| match position {
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => State::Floor,
                })
                .collect()
        })
        .collect()
}

fn count_map(map: &Map, count_state: State) -> usize {
    map.par_iter()
        .map(|columns| {
            columns
                .par_iter()
                .filter(|state| state == &&count_state)
                .count()
        })
        .sum()
}

fn state_map(map: &Map, row: i32, column: i32) -> Option<&State> {
    map.get(row as usize)
        .map(|columns| columns.get(column as usize))
        .flatten()
}

fn stabilize_map(map: Map, adjacent_count: usize, adjacent_immediately: bool) -> Map {
    let mut current_state = map;

    loop {
        let next_state = next_map_state(&current_state, adjacent_count, adjacent_immediately);

        if next_state == current_state {
            return current_state;
        } else {
            current_state = next_state;
        }
    }
}

fn next_map_state(map: &Map, adjacent_count: usize, adjacent_immediately: bool) -> Map {
    map.par_iter()
        .enumerate()
        .map(|(row, columns)| {
            columns
                .par_iter()
                .enumerate()
                .map(|(column, state)| {
                    next_position_state(
                        map,
                        row as i32,
                        column as i32,
                        state,
                        adjacent_count,
                        adjacent_immediately,
                    )
                })
                .collect()
        })
        .collect()
}

fn next_position_state(
    map: &Map,
    row: i32,
    column: i32,
    state: &State,
    adjacent_count: usize,
    adjacent_immediately: bool,
) -> State {
    if state == &State::Floor {
        State::Floor
    } else {
        let count_directions: Vec<(i32, i32)> = (-1..2)
            .flat_map(|y| (-1..2).map(move |x| (y, x)))
            .filter(|(y, x)| y != &0 || x != &0)
            .collect();

        let occupied_count = if adjacent_immediately {
            count_adjacent_immediately(map, row, column, count_directions)
        } else {
            count_adjacent_directions(map, row, column, count_directions)
        };

        if state == &State::Empty && occupied_count == 0 {
            State::Occupied
        } else if state == &State::Occupied && occupied_count >= adjacent_count {
            State::Empty
        } else {
            *state
        }
    }
}

fn count_adjacent_immediately(
    map: &Map,
    row: i32,
    column: i32,
    directions: Vec<(i32, i32)>,
) -> usize {
    directions
        .par_iter()
        .map(|(y, x)| state_map(map, row + y, column + x))
        .filter_map(|state| state)
        .filter(|state| **state == State::Occupied)
        .count()
}

fn count_adjacent_directions(
    map: &Map,
    row: i32,
    column: i32,
    directions: Vec<(i32, i32)>,
) -> usize {
    directions
        .par_iter()
        .map(|(y, x)| {
            let mut state = None;

            for i in 1.. {
                state = state_map(map, row + y * i, column + x * i);

                match state {
                    Some(State::Floor) => (),
                    _ => break,
                }
            }

            state
        })
        .filter_map(|state| state)
        .filter(|state| **state == State::Occupied)
        .count()
}
