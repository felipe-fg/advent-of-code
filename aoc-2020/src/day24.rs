use aoc_utils;
use regex::Regex;
use std::collections::HashMap;

type Floor = HashMap<AxialCoordinate, bool>;
type Value = i32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct AxialCoordinate {
    q: Value,
    r: Value,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl AsRef<Self> for Direction {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day24.txt", true).collect();

    let steps = parse_steps(&lines);

    let initial_floor = build_initial(&steps);
    let daily_floor = build_daily(&initial_floor, 100);

    let count_initial = count_tiles(&initial_floor);
    let count_daily = count_tiles(&daily_floor);

    println!("{:?}", count_initial);
    println!("{:?}", count_daily);
}

fn parse_steps(lines: &[String]) -> Vec<Vec<Direction>> {
    let re = Regex::new(r"se|sw|nw|ne|e|w").expect("regex");

    lines
        .iter()
        .map(|line| {
            re.find_iter(line)
                .map(|step| match step.as_str() {
                    "e" => Direction::East,
                    "se" => Direction::Southeast,
                    "sw" => Direction::Southwest,
                    "w" => Direction::West,
                    "nw" => Direction::Northwest,
                    "ne" => Direction::Northeast,
                    _ => Direction::East,
                })
                .collect()
        })
        .collect()
}

fn build_initial<T, U>(steps: &[U]) -> Floor
where
    T: AsRef<Direction>,
    U: AsRef<[T]>,
{
    let mut floor: Floor = HashMap::new();

    for directions in steps {
        let mut position = AxialCoordinate { q: 0, r: 0 };

        for direction in directions.as_ref() {
            position = next_position(&position, direction.as_ref());
        }

        floor
            .entry(position)
            .and_modify(|black| *black = !*black)
            .or_insert(true);
    }

    floor
}

fn build_daily(initial_floor: &Floor, days: usize) -> Floor {
    let mut floor = initial_floor.clone();

    let mut min_q = floor.keys().map(|tile| tile.q).min().expect("min q") - 1;
    let mut max_q = floor.keys().map(|tile| tile.q).max().expect("max q") + 1;
    let mut min_r = floor.keys().map(|tile| tile.r).min().expect("min r") - 1;
    let mut max_r = floor.keys().map(|tile| tile.r).max().expect("max r") + 1;

    for _ in 0..days {
        floor = next_floor_state(&floor, (min_q, max_q, min_r, max_r));

        min_q -= 1;
        max_q += 1;
        min_r -= 1;
        max_r += 1;
    }

    floor
}

fn next_floor_state(
    floor: &Floor,
    (min_q, max_q, min_r, max_r): (Value, Value, Value, Value),
) -> Floor {
    let mut next_floor = floor.clone();

    for q in min_q..(max_q + 1) {
        for r in min_r..(max_r + 1) {
            let position = AxialCoordinate { q, r };

            let black = floor.get(&position).map(|black| *black).unwrap_or(false);
            let neighbors = count_neighbors(&position, &floor);

            if black && (neighbors == 0 || neighbors > 2) {
                next_floor
                    .entry(position)
                    .and_modify(|black| *black = false);
            } else if !black && neighbors == 2 {
                next_floor
                    .entry(position)
                    .and_modify(|black| *black = true)
                    .or_insert(true);
            }
        }
    }

    next_floor
}

fn count_neighbors(position: &AxialCoordinate, floor: &Floor) -> usize {
    vec![
        Direction::East,
        Direction::Southeast,
        Direction::Southwest,
        Direction::West,
        Direction::Northwest,
        Direction::Northeast,
    ]
    .iter()
    .map(|direction| {
        let position = next_position(position, direction);

        floor.get(&position).map(|black| *black).unwrap_or(false)
    })
    .filter(|black| *black)
    .count()
}

fn count_tiles(floor: &Floor) -> usize {
    floor.iter().filter(|(_, black)| **black).count()
}

fn next_position(position: &AxialCoordinate, direction: &Direction) -> AxialCoordinate {
    match direction {
        Direction::East => AxialCoordinate {
            q: position.q + 1,
            r: position.r + 0,
        },
        Direction::Southeast => AxialCoordinate {
            q: position.q + 0,
            r: position.r + 1,
        },
        Direction::Southwest => AxialCoordinate {
            q: position.q - 1,
            r: position.r + 1,
        },
        Direction::West => AxialCoordinate {
            q: position.q - 1,
            r: position.r + 0,
        },
        Direction::Northwest => AxialCoordinate {
            q: position.q + 0,
            r: position.r - 1,
        },
        Direction::Northeast => AxialCoordinate {
            q: position.q + 1,
            r: position.r - 1,
        },
    }
}
