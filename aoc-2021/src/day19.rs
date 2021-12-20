use aoc_utils;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;

type Value = i64;
type Rotation = Box<dyn Fn(&Point) -> Point>;

const SCANNER_MATCH: usize = 12;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: Value,
    y: Value,
    z: Value,
}

impl Point {
    fn new(x: Value, y: Value, z: Value) -> Self {
        Self { x, y, z }
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    fn distance(&self, other: &Self) -> Value {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn rotate(&self, rotation: &Rotation) -> Point {
        rotation(self)
    }

    fn rotations() -> Vec<Rotation> {
        vec![
            Box::new(|&Point { x, y, z }| Point::new(x, y, z)),
            Box::new(|&Point { x, y, z }| Point::new(x, -y, -z)),
            Box::new(|&Point { x, y, z }| Point::new(x, z, -y)),
            Box::new(|&Point { x, y, z }| Point::new(x, -z, y)),
            Box::new(|&Point { x, y, z }| Point::new(-x, -z, -y)),
            Box::new(|&Point { x, y, z }| Point::new(-x, z, y)),
            Box::new(|&Point { x, y, z }| Point::new(-x, -y, z)),
            Box::new(|&Point { x, y, z }| Point::new(-x, y, -z)),
            Box::new(|&Point { x, y, z }| Point::new(y, z, x)),
            Box::new(|&Point { x, y, z }| Point::new(y, -z, -x)),
            Box::new(|&Point { x, y, z }| Point::new(y, x, -z)),
            Box::new(|&Point { x, y, z }| Point::new(y, -x, z)),
            Box::new(|&Point { x, y, z }| Point::new(-y, -x, -z)),
            Box::new(|&Point { x, y, z }| Point::new(-y, x, z)),
            Box::new(|&Point { x, y, z }| Point::new(-y, -z, x)),
            Box::new(|&Point { x, y, z }| Point::new(-y, z, -x)),
            Box::new(|&Point { x, y, z }| Point::new(z, x, y)),
            Box::new(|&Point { x, y, z }| Point::new(z, -x, -y)),
            Box::new(|&Point { x, y, z }| Point::new(z, y, -x)),
            Box::new(|&Point { x, y, z }| Point::new(z, -y, x)),
            Box::new(|&Point { x, y, z }| Point::new(-z, -y, -x)),
            Box::new(|&Point { x, y, z }| Point::new(-z, y, x)),
            Box::new(|&Point { x, y, z }| Point::new(-z, -x, y)),
            Box::new(|&Point { x, y, z }| Point::new(-z, x, -y)),
        ]
    }
}

#[derive(Debug, Clone)]
struct ScannerMatch {
    id: String,
    position: Point,
    distances: Vec<Distance>,
    global_beacons: Vec<Point>,
    local_beacons: Vec<Point>,
}

impl ScannerMatch {
    fn from_scanner(scanner: &Scanner, position: &Point) -> ScannerMatch {
        let id = scanner.id.clone();
        let position = position.clone();
        let distances = scanner.distances.clone();

        let global_beacons = scanner
            .beacons
            .iter()
            .map(|beacon| position.add(beacon))
            .collect();

        let local_beacons = scanner
            .beacons
            .iter()
            .map(|beacon| beacon.clone())
            .collect();

        Self {
            id,
            position,
            distances,
            global_beacons,
            local_beacons,
        }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: String,
    beacons: Vec<Point>,
    distances: Vec<Distance>,
    rotations: Vec<Box<Scanner>>,
}

impl Scanner {
    fn new(id: String, beacons: Vec<Point>, rotate: bool) -> Self {
        let distances = Distance::from_beacons(&beacons);

        let rotations = if rotate {
            Self::from_rotations(&id, &beacons)
        } else {
            vec![]
        };

        Self {
            id,
            beacons,
            distances,
            rotations,
        }
    }

    fn from_rotations(id: &str, beacons: &[Point]) -> Vec<Box<Scanner>> {
        Point::rotations()
            .iter()
            .map(|rotation| Self::from_rotation(id, beacons, rotation))
            .collect()
    }

    fn from_rotation(id: &str, beacons: &[Point], rotation: &Rotation) -> Box<Scanner> {
        let id = id.to_string();

        let beacons = beacons
            .iter()
            .map(|beacon| beacon.rotate(rotation))
            .collect();

        let scanner = Scanner::new(id, beacons, false);

        Box::new(scanner)
    }
}

#[derive(Debug, Clone)]
struct Distance {
    origin: Point,
    vectors: Vec<Point>,
}

impl Distance {
    fn new(origin: Point, vectors: Vec<Point>) -> Self {
        Self { origin, vectors }
    }

    fn from_beacons(beacons: &[Point]) -> Vec<Self> {
        beacons
            .iter()
            .map(|origin| Self::from_origin_beacons(origin, beacons))
            .collect()
    }

    fn from_origin_beacons(&origin: &Point, beacons: &[Point]) -> Self {
        let vectors = beacons.iter().map(|beacon| beacon.sub(&origin)).collect();

        Self::new(origin, vectors)
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day19.txt", true).collect();

    let scanners = parse_scanners(lines);

    let (position_beacons, position_scanners) = scanner_match(&scanners);

    let distance_scanners = find_largest_distance(&position_scanners);

    println!("{}", position_beacons.len());
    println!("{}", distance_scanners);
}

fn parse_scanners(lines: Vec<String>) -> Vec<Scanner> {
    let re_id = Regex::new(r"--- scanner (?P<id>.+) ---").expect("regex id");
    let re_point = Regex::new(r"(?P<x>.+),(?P<y>.+),(?P<z>.+)").expect("regex point");

    let mut scanners = vec![];

    let mut current_id = String::from("");
    let mut current_beacons = vec![];

    for line in lines {
        if line.contains("scanner") {
            if !current_beacons.is_empty() {
                let scanner = Scanner::new(current_id, current_beacons, true);

                scanners.push(scanner);
            }

            let caps = re_id.captures(&line).expect("captures");
            let id = caps["id"].trim().to_string();

            current_id = id;
            current_beacons = vec![];
        } else {
            let caps = re_point.captures(&line).expect("captures");
            let x = caps["x"].trim().to_string().parse().expect("number");
            let y = caps["y"].trim().to_string().parse().expect("number");
            let z = caps["z"].trim().to_string().parse().expect("number");

            let beacon = Point::new(x, y, z);

            current_beacons.push(beacon);
        }
    }

    if !current_beacons.is_empty() {
        let scanner = Scanner::new(current_id, current_beacons, true);

        scanners.push(scanner);
    }

    scanners
}

fn scanner_match(scanners: &[Scanner]) -> (HashSet<Point>, HashSet<Point>) {
    let mut scanners: Vec<Scanner> = scanners.iter().map(|scanner| scanner.clone()).collect();

    let mut position_beacons: HashSet<Point> = HashSet::new();
    let mut position_scanners: HashSet<Point> = HashSet::new();
    let mut scanner_matches: Vec<ScannerMatch> = vec![];

    let scanner = scanners.remove(0);
    let position = Point::new(0, 0, 0);
    let scanner_match = ScannerMatch::from_scanner(&scanner, &position);

    position_beacons.extend(scanner_match.global_beacons.iter());
    position_scanners.insert(scanner_match.position.clone());
    scanner_matches.push(scanner_match);

    while !scanners.is_empty() {
        let scanner_match = scanners
            .iter()
            .find_map(|scanner| {
                scanner_matches
                    .par_iter()
                    .find_map_any(|scanner_match| try_scanner_match(scanner_match, scanner))
            })
            .expect("match");

        scanners.retain(|scanner| scanner.id != scanner_match.id);

        position_beacons.extend(scanner_match.global_beacons.iter());
        position_scanners.insert(scanner_match.position.clone());
        scanner_matches.push(scanner_match);
    }

    (position_beacons, position_scanners)
}

fn try_scanner_match(left_scanner: &ScannerMatch, right_scanner: &Scanner) -> Option<ScannerMatch> {
    for right_scanner in &right_scanner.rotations {
        for right_distance in &right_scanner.distances {
            for left_distance in &left_scanner.distances {
                let match_count = left_distance
                    .vectors
                    .iter()
                    .filter(|left_vector| right_distance.vectors.contains(left_vector))
                    .count();

                if match_count >= SCANNER_MATCH {
                    let right_offset = left_distance.origin.sub(&right_distance.origin);
                    let right_position = left_scanner.position.add(&right_offset);

                    let scanner_match = ScannerMatch::from_scanner(right_scanner, &right_position);

                    return Some(scanner_match);
                }
            }
        }
    }

    None
}

fn find_largest_distance(positions: &HashSet<Point>) -> Value {
    positions
        .iter()
        .permutations(2)
        .map(|pair| pair[0].distance(pair[1]))
        .max()
        .expect("max")
}
