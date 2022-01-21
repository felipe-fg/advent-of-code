use aoc_utils;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap};

type Unit = usize;
type Cost = usize;

const DIAGRAM_HALLWAY: char = '.';
const DIAGRAM_A: char = 'A';
const DIAGRAM_B: char = 'B';
const DIAGRAM_C: char = 'C';
const DIAGRAM_D: char = 'D';

const COST_MAX: Cost = Cost::MAX;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl TryFrom<char> for Amphipod {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            DIAGRAM_A => Ok(Self::A),
            DIAGRAM_B => Ok(Self::B),
            DIAGRAM_C => Ok(Self::C),
            DIAGRAM_D => Ok(Self::D),
            _ => Err(format!("{} is not a valid amphipod.", value)),
        }
    }
}

impl Amphipod {
    fn all() -> Vec<Self> {
        vec![Self::A, Self::B, Self::C, Self::D]
    }

    fn cost(&self) -> Cost {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: Unit,
    y: Unit,
}

impl Point {
    fn new(x: Unit, y: Unit) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    hallway: Vec<Point>,
    rooms: HashMap<Amphipod, Vec<Point>>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            hallway: Vec::new(),
            rooms: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    amphipods: BTreeSet<(Point, Amphipod)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            amphipods: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SearchState {
    state: State,
    cost: Cost,
}

impl SearchState {
    fn new(state: State, cost: Cost) -> Self {
        Self { state, cost }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run() {
    let lines_folded: Vec<String> = aoc_utils::read_lines("inputs/day23.txt", true).collect();

    let lines_unfolded = unfold_diagram(&lines_folded);

    let (map_folded, state_folded) = parse_diagram(&lines_folded);
    let (map_unfolded, state_unfolded) = parse_diagram(&lines_unfolded);

    let cost_folded = dijkstra_search(&map_folded, &state_folded);
    let cost_unfolded = dijkstra_search(&map_unfolded, &state_unfolded);

    println!("{}", cost_folded.expect("cost"));
    println!("{}", cost_unfolded.expect("cost"));
}

fn unfold_diagram(lines: &[String]) -> Vec<String> {
    let mut new_lines: Vec<String> = lines.iter().map(|line| line.clone()).collect();

    new_lines.insert(3, "  #D#C#B#A#".to_string());
    new_lines.insert(4, "  #D#B#A#C#".to_string());

    new_lines
}

fn parse_diagram(lines: &[String]) -> (Map, State) {
    let mut map = Map::default();
    let mut state = State::default();

    let mut cycle = Amphipod::all().into_iter().cycle();

    for (row, line) in lines.iter().enumerate() {
        for (column, tile) in line.chars().enumerate() {
            let point = Point::new(column - 1, row - 1);

            if tile == DIAGRAM_HALLWAY {
                let down = lines[row + 1].chars().nth(column).expect("down");

                if let Err(_) = Amphipod::try_from(down) {
                    map.hallway.push(point);
                }
            } else if let Ok(amphipod) = Amphipod::try_from(tile) {
                let room = cycle.next().expect("room");

                map.rooms.entry(room).or_insert(Vec::new()).push(point);

                state.amphipods.insert((point, amphipod));
            }
        }
    }

    (map, state)
}

fn dijkstra_search(map: &Map, initial_state: &State) -> Option<Cost> {
    let mut heap: BinaryHeap<SearchState> = BinaryHeap::new();
    let mut distance: HashMap<State, Cost> = HashMap::new();

    heap.push(SearchState::new(initial_state.clone(), 0));
    distance.insert(initial_state.clone(), 0);

    while let Some(SearchState { state, cost }) = heap.pop() {
        if is_completed(&map, &state) {
            return Some(cost);
        }

        for (next, next_cost_state) in get_next_states(&map, &state) {
            let new_next_cost = cost + next_cost_state;

            let next_cost = distance.get(&next).map(|&cost| cost).unwrap_or(COST_MAX);

            if new_next_cost < next_cost {
                heap.push(SearchState::new(next.clone(), new_next_cost));
                distance.insert(next.clone(), new_next_cost);
            }
        }
    }

    None
}

fn is_completed(map: &Map, state: &State) -> bool {
    state.amphipods.iter().all(|(point, amphipod)| {
        map.rooms
            .get(amphipod)
            .filter(|points| points.contains(point))
            .is_some()
    })
}

fn get_next_states(map: &Map, state: &State) -> Vec<(State, Cost)> {
    state
        .amphipods
        .iter()
        .flat_map(|amphipod| get_amphipod_next_states(map, state, amphipod))
        .collect()
}

fn get_amphipod_next_states(
    map: &Map,
    state: &State,
    (point, amphipod): &(Point, Amphipod),
) -> Vec<(State, Cost)> {
    let is_hallway = map.hallway.contains(point);

    if is_hallway {
        let next_room = get_amphipod_next_room(map, state, amphipod);

        next_room
            .map(|room| move_amphipod(state, point, &room, amphipod))
            .flatten()
            .map(|next_state| vec![next_state])
            .unwrap_or(vec![])
    } else {
        let should_move = should_amphipod_move_hallway(map, state, point);

        map.hallway
            .iter()
            .filter(|_| should_move)
            .flat_map(|hallway| move_amphipod(state, point, &hallway, amphipod))
            .collect()
    }
}

fn get_amphipod_next_room(map: &Map, state: &State, room_amphipod: &Amphipod) -> Option<Point> {
    let room_points = map.rooms.get(room_amphipod).expect("room");

    let room_amphipods = join_amphipods(state, room_points);

    room_amphipods
        .iter()
        .rev()
        .take_while(|&&(_, amphipod)| amphipod.is_none() || amphipod == Some(room_amphipod.clone()))
        .filter(|(_, room_amphipod)| room_amphipod.is_none())
        .next()
        .map(|(point, _)| point.clone())
}

fn should_amphipod_move_hallway(map: &Map, state: &State, point: &Point) -> bool {
    let (room_amphipod, room_points) = map
        .rooms
        .iter()
        .find(|(_, points)| points.contains(point))
        .expect("room");

    let room_amphipods = join_amphipods(state, room_points);

    let should_keep = room_amphipods
        .iter()
        .rev()
        .take_while(|&&(_, amphipod)| amphipod == Some(room_amphipod.clone()))
        .any(|(room_point, _)| room_point == point);

    !should_keep
}

fn join_amphipods(state: &State, points: &Vec<Point>) -> Vec<(Point, Option<Amphipod>)> {
    points
        .iter()
        .map(|point| {
            let amphipod = state
                .amphipods
                .iter()
                .find(|(amphipod_point, _)| amphipod_point == point)
                .map(|(_, amphipod)| amphipod.clone());

            (point.clone(), amphipod)
        })
        .collect()
}

fn move_amphipod(
    state: &State,
    start: &Point,
    end: &Point,
    amphipod: &Amphipod,
) -> Option<(State, Cost)> {
    let movement = get_movement(state, start, &end, amphipod);

    movement.map(|cost| {
        let mut next_state = state.clone();

        next_state.amphipods.retain(|(point, _)| point != start);
        next_state.amphipods.insert((end.clone(), amphipod.clone()));

        (next_state, cost)
    })
}

fn get_movement(state: &State, start: &Point, end: &Point, amphipod: &Amphipod) -> Option<Cost> {
    let down = end.y >= start.y;

    let points: Vec<Point> = if down {
        let horizontal = range(start.x, end.x).map(|x| Point::new(x, start.y));
        let vertical = range(start.y, end.y).map(|y| Point::new(end.x, y));

        horizontal.chain(vertical).dedup().collect()
    } else {
        let vertical = range(start.y, end.y).map(|y| Point::new(start.x, y));
        let horizontal = range(start.x, end.x).map(|x| Point::new(x, end.y));

        vertical.chain(horizontal).dedup().collect()
    };

    let collision = state
        .amphipods
        .iter()
        .any(|(point, _)| point != start && points.contains(point));

    if collision {
        None
    } else {
        Some((points.len() - 1) * amphipod.cost())
    }
}

fn range(from: usize, to: usize) -> Box<dyn Iterator<Item = usize>> {
    if to >= from {
        Box::new(from..=to)
    } else {
        Box::new((to..=from).rev())
    }
}
