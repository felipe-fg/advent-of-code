use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use colored::*;

type Loss = u64;
type Map = Vec<Vec<Loss>>;
type Number = isize;
type Position = (Number, Number);

const LOSS_MAX: Loss = Loss::MAX;
const CRUCIBLE_MIN: Number = 0;
const CRUCIBLE_MAX: Number = 3;
const ULTRA_CRUCIBLE_MIN: Number = 4;
const ULTRA_CRUCIBLE_MAX: Number = 10;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    loss: Loss,
    direction: Direction,
    steps: Number,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct StateKey {
    position: Position,
    direction: Direction,
    steps: Number,
}

impl From<State> for StateKey {
    fn from(
        State {
            position,
            direction,
            steps,
            ..
        }: State,
    ) -> Self {
        Self {
            position,
            direction,
            steps,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day17.txt", true).collect();

    let map = parse_map(&lines);
    let rows = map.len();
    let columns = map.first().map(|row| row.len()).unwrap_or_default();

    let start = (0, 0);
    let goal = (columns as Number - 1, rows as Number - 1);
    let path = dijkstra_search(&map, start, goal, CRUCIBLE_MIN, CRUCIBLE_MAX);
    let path_ultra = dijkstra_search(&map, start, goal, ULTRA_CRUCIBLE_MIN, ULTRA_CRUCIBLE_MAX);

    draw_map(&map, &path);
    println!();
    draw_map(&map, &path_ultra);

    println!("{}", path.last().expect("path").loss);
    println!("{}", path_ultra.last().expect("path ultra").loss);
}

fn parse_map(lines: &[String]) -> Map {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_string().parse().ok())
                .collect()
        })
        .collect()
}

fn dijkstra_search(
    map: &Map,
    start: Position,
    goal: Position,
    min: Number,
    max: Number,
) -> Vec<State> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut distance: HashMap<StateKey, Loss> = HashMap::new();
    let mut previous: HashMap<StateKey, Option<State>> = HashMap::new();
    let mut goal_state: Option<State> = None;

    let initial_states = vec![
        State {
            position: start,
            loss: 0,
            direction: Direction::Right,
            steps: 0,
        },
        State {
            position: start,
            loss: 0,
            direction: Direction::Down,
            steps: 0,
        },
    ];

    for initial_state in initial_states {
        heap.push(initial_state);
        distance.insert(initial_state.into(), initial_state.loss);
        previous.insert(initial_state.into(), None);
    }

    while let Some(state) = heap.pop() {
        if state.position == goal && state.steps >= min {
            goal_state = Some(state);
            break;
        }

        let neighbors = get_neighbors(map, &state, min, max);

        for neighbor in neighbors {
            let loss = distance.get(&neighbor.into()).copied().unwrap_or(LOSS_MAX);

            if neighbor.loss < loss {
                heap.push(neighbor);
                distance.insert(neighbor.into(), neighbor.loss);
                previous.insert(neighbor.into(), Some(state));
            }
        }
    }

    let mut path: Vec<State> = Vec::new();
    let mut path_state = goal_state;

    while let Some(state) = path_state {
        let previous_state = previous.get(&state.into()).copied().flatten();

        path_state = previous_state;
        path.push(state);
    }

    path.reverse();

    path
}

fn get_neighbors(map: &Map, current: &State, min: Number, max: Number) -> Vec<State> {
    get_directions(current, min, max)
        .into_iter()
        .map(|(direction, steps)| {
            let position = get_position(current.position, direction);

            (position, direction, steps)
        })
        .filter(|(position, _, _)| is_position_valid(map, *position))
        .map(|(position, direction, steps)| {
            let loss = current.loss + get_loss(map, position);

            State {
                position,
                loss,
                direction,
                steps,
            }
        })
        .collect()
}

fn get_directions(current: &State, min: Number, max: Number) -> Vec<(Direction, Number)> {
    let must_keep = current.steps < min;
    let must_turn = current.steps >= max;

    match current.direction {
        Direction::Up if must_keep => vec![(Direction::Up, current.steps + 1)],
        Direction::Down if must_keep => vec![(Direction::Down, current.steps + 1)],
        Direction::Left if must_keep => vec![(Direction::Left, current.steps + 1)],
        Direction::Right if must_keep => vec![(Direction::Right, current.steps + 1)],
        Direction::Up | Direction::Down if must_turn => {
            vec![(Direction::Left, 1), (Direction::Right, 1)]
        }
        Direction::Left | Direction::Right if must_turn => {
            vec![(Direction::Up, 1), (Direction::Down, 1)]
        }
        Direction::Up => vec![
            (Direction::Up, current.steps + 1),
            (Direction::Left, 1),
            (Direction::Right, 1),
        ],
        Direction::Down => vec![
            (Direction::Down, current.steps + 1),
            (Direction::Left, 1),
            (Direction::Right, 1),
        ],
        Direction::Left => vec![
            (Direction::Left, current.steps + 1),
            (Direction::Up, 1),
            (Direction::Down, 1),
        ],
        Direction::Right => vec![
            (Direction::Right, current.steps + 1),
            (Direction::Up, 1),
            (Direction::Down, 1),
        ],
    }
}

fn get_position((x, y): Position, direction: Direction) -> Position {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn get_loss(map: &Map, (x, y): Position) -> Loss {
    if is_position_valid(map, (x, y)) {
        map.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .unwrap_or(LOSS_MAX)
    } else {
        LOSS_MAX
    }
}

fn is_position_valid(map: &Map, (x, y): Position) -> bool {
    let rows = map.len();
    let columns = map.first().map(|row| row.len()).unwrap_or_default();

    x >= 0 && x < columns as Number && y >= 0 && y < rows as Number
}

fn draw_map(map: &Map, path: &[State]) {
    for (row_index, row) in map.iter().enumerate() {
        for (column_index, &loss) in row.iter().enumerate() {
            let position = (column_index as Number, row_index as Number);
            let is_path = path.iter().any(|state| state.position == position);
            let brightness = 255 - ((loss as u8 - 1) * 31);

            let tile = if is_path {
                "░".truecolor(255, 255, 255).on_truecolor(brightness, 0, 0)
            } else {
                "█".truecolor(brightness, 0, 0).on_truecolor(0, 0, 0)
            };

            print!("{}", tile);
        }

        println!();
    }
}
