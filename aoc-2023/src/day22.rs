use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

type ID = usize;
type Number = i64;
type Bricks = Vec<Brick>;
type BricksByZ = HashMap<Number, Vec<Brick>>;
type SupportTree = HashMap<ID, HashSet<ID>>;

const GROUND_LEVEL: Number = 0;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: Number,
    y: Number,
    z: Number,
}

impl Display for Position {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Brick {
    id: ID,
    start: Position,
    end: Position,
}

impl Brick {
    fn move_down(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
    }

    fn move_up(&mut self) {
        self.start.z += 1;
        self.end.z += 1;
    }
}

impl Display for Brick {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}:{}~{}", self.id, self.start, self.end)
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day22.txt", true).collect();

    let bricks = parse_bricks(&lines);

    let (stable_bricks, stable_bricks_by_z) = process_falling_bricks(&bricks);

    let support_tree = build_support_tree(&stable_bricks, &stable_bricks_by_z);

    let optional_bricks = get_optional_bricks(&support_tree);

    let count_chain_reaction = count_chain_reaction_bricks(&optional_bricks, &stable_bricks);

    println!("{}", optional_bricks.len());
    println!("{}", count_chain_reaction);
}

fn parse_bricks(lines: &[String]) -> Bricks {
    let re = r"(?P<x0>\d+),(?P<y0>\d+),(?P<z0>\d+)~(?P<x1>\d+),(?P<y1>\d+),(?P<z1>\d+)";
    let re = Regex::new(re).expect("regex");

    lines
        .iter()
        .enumerate()
        .map(|(id, line)| {
            let caps = re.captures(line).expect("captures");

            let start = Position {
                x: caps["x0"].parse().expect("x0"),
                y: caps["y0"].parse().expect("y0"),
                z: caps["z0"].parse().expect("z0"),
            };

            let end = Position {
                x: caps["x1"].parse().expect("x1"),
                y: caps["y1"].parse().expect("y1"),
                z: caps["z1"].parse().expect("z1"),
            };

            Brick { id, start, end }
        })
        .collect()
}

fn process_falling_bricks(falling_bricks: &Bricks) -> (Bricks, BricksByZ) {
    let mut sorted_falling_bricks = falling_bricks.to_vec();
    sorted_falling_bricks.sort_by_key(|brick| brick.end.z.min(brick.start.z));

    let mut stable_bricks_by_z = BricksByZ::new();

    for falling_brick in sorted_falling_bricks {
        let stable_brick = process_falling_brick(&falling_brick, &stable_bricks_by_z);

        let min_z = stable_brick.start.z.min(stable_brick.end.z);
        let max_z = stable_brick.start.z.max(stable_brick.end.z);

        for z in min_z..=max_z {
            stable_bricks_by_z.entry(z).or_default().push(stable_brick);
        }
    }

    let stable_bricks = stable_bricks_by_z
        .iter()
        .flat_map(|(_, stable_bricks)| stable_bricks)
        .unique()
        .copied()
        .collect();

    (stable_bricks, stable_bricks_by_z)
}

fn process_falling_brick(falling_brick: &Brick, bricks_by_z: &BricksByZ) -> Brick {
    let mut falling_brick = *falling_brick;

    while !is_colliding(&falling_brick, bricks_by_z) {
        falling_brick.move_down()
    }

    falling_brick.move_up();

    falling_brick
}

fn build_support_tree(stable_bricks: &Bricks, bricks_by_z: &BricksByZ) -> SupportTree {
    let mut support_tree = SupportTree::new();

    for stable_brick in stable_bricks {
        let mut supporting_brick = *stable_brick;
        supporting_brick.move_up();

        let supporting_brick_entry = support_tree.entry(supporting_brick.id).or_default();

        let supported_bricks = get_colliding_bricks(&supporting_brick, bricks_by_z);

        for supported_brick in supported_bricks {
            supporting_brick_entry.insert(supported_brick.id);
        }
    }

    support_tree
}

fn get_optional_bricks(support_tree: &SupportTree) -> Vec<ID> {
    support_tree
        .iter()
        .filter(|(&supporting_id, supported)| {
            let all_supported_by_others = supported.iter().all(|supported_id| {
                is_supported_by_other(supporting_id, *supported_id, support_tree)
            });

            supported.is_empty() || all_supported_by_others
        })
        .map(|(&supporting_id, _)| supporting_id)
        .collect()
}

fn count_chain_reaction_bricks(optional_bricks: &[ID], stable_bricks: &Bricks) -> usize {
    let required_bricks: Bricks = stable_bricks
        .iter()
        .filter(|stable_brick| !optional_bricks.contains(&stable_brick.id))
        .copied()
        .collect();

    required_bricks
        .par_iter()
        .map(|&required_brick| count_chain_reaction_brick(&required_brick, stable_bricks))
        .sum()
}

fn count_chain_reaction_brick(required_brick: &Brick, stable_bricks: &Bricks) -> usize {
    let mut next_bricks = stable_bricks.to_vec();
    next_bricks.retain(|brick| brick != required_brick);

    let (next_stable_bricks, _) = process_falling_bricks(&next_bricks);

    let unchanged_bricks = next_stable_bricks
        .iter()
        .filter(|next_stable_brick| stable_bricks.contains(next_stable_brick))
        .count();

    next_bricks.len() - unchanged_bricks
}

fn is_supported_by_other(supporting_id: ID, supported_id: ID, support_tree: &SupportTree) -> bool {
    support_tree
        .iter()
        .filter(|(&other_supporting_id, _)| other_supporting_id != supporting_id)
        .any(|(_, other_supported)| other_supported.contains(&supported_id))
}

fn is_colliding(brick: &Brick, stable_bricks_by_z: &BricksByZ) -> bool {
    let is_colliding_ground = is_colliding_ground(brick);

    let stable_bricks = get_bricks_nearby(brick, stable_bricks_by_z);

    let is_colliding_stable = stable_bricks
        .iter()
        .filter(|stable_brick| stable_brick.id != brick.id)
        .any(|stable_brick| is_colliding_brick(brick, stable_brick));

    is_colliding_ground || is_colliding_stable
}

fn get_colliding_bricks(brick: &Brick, stable_bricks_by_z: &BricksByZ) -> Bricks {
    let stable_bricks = get_bricks_nearby(brick, stable_bricks_by_z);

    stable_bricks
        .iter()
        .filter(|stable_brick| stable_brick.id != brick.id)
        .filter(|stable_brick| is_colliding_brick(brick, stable_brick))
        .copied()
        .collect()
}

fn get_bricks_nearby(brick: &Brick, stable_bricks_by_z: &BricksByZ) -> Bricks {
    let min_z = brick.start.z.min(brick.end.z);
    let max_z = brick.start.z.max(brick.end.z);

    (min_z..=max_z)
        .filter_map(|z| stable_bricks_by_z.get(&z))
        .flatten()
        .copied()
        .collect()
}

fn is_colliding_brick(a: &Brick, b: &Brick) -> bool {
    let a_x = (a.start.x, a.end.x);
    let a_y = (a.start.y, a.end.y);
    let a_z = (a.start.z, a.end.z);

    let b_x = (b.start.x, b.end.x);
    let b_y = (b.start.y, b.end.y);
    let b_z = (b.start.z, b.end.z);

    let is_colliding_x = is_colliding_interval(&a_x, &b_x);
    let is_colliding_y = is_colliding_interval(&a_y, &b_y);
    let is_colliding_z = is_colliding_interval(&a_z, &b_z);

    is_colliding_x && is_colliding_y && is_colliding_z
}

fn is_colliding_interval(
    (start_a, end_a): &(Number, Number),
    (start_b, end_b): &(Number, Number),
) -> bool {
    let min_a = start_a.min(end_a);
    let max_a = start_a.max(end_a);

    let min_b = start_b.min(end_b);
    let max_b = start_b.max(end_b);

    !(max_a < min_b || min_a > max_b)
}

fn is_colliding_ground(a: &Brick) -> bool {
    a.start.z <= GROUND_LEVEL || a.end.z <= GROUND_LEVEL
}
