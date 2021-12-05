use aoc_utils;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct LineSegment {
    a: Point,
    b: Point,
}

impl LineSegment {
    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_diagonal(&self) -> bool {
        (self.b.x - self.a.x).abs() == (self.b.y - self.a.y).abs()
    }

    fn trace(&self) -> Vec<Point> {
        let direction_x = self.b.x.cmp(&self.a.x) as isize;
        let direction_y = self.b.y.cmp(&self.a.y) as isize;

        let steps = (self.b.x - self.a.x).abs().max((self.b.y - self.a.y).abs());

        (0..steps + 1)
            .map(|index| Point {
                x: self.a.x + (direction_x * index),
                y: self.a.y + (direction_y * index),
            })
            .collect()
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day05.txt", true).collect();

    let line_segments = parse_line_segments(lines);

    let points = trace_line_segments(&line_segments, false);
    let points_diagonal = trace_line_segments(&line_segments, true);

    let overlap_points = count_overlap_points(&points);
    let overlap_points_diagonal = count_overlap_points(&points_diagonal);

    println!("{}", overlap_points);
    println!("{}", overlap_points_diagonal);
}

fn parse_line_segments(lines: Vec<String>) -> Vec<LineSegment> {
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").expect("regex");

    lines
        .par_iter()
        .map(|line| {
            let caps = re.captures(line).expect("captures");
            let x1 = caps["x1"].to_string().parse().expect("number");
            let y1 = caps["y1"].to_string().parse().expect("number");
            let x2 = caps["x2"].to_string().parse().expect("number");
            let y2 = caps["y2"].to_string().parse().expect("number");

            LineSegment {
                a: Point { x: x1, y: y1 },
                b: Point { x: x2, y: y2 },
            }
        })
        .collect()
}

fn trace_line_segments(line_segments: &[LineSegment], diagonal: bool) -> Vec<Point> {
    line_segments
        .par_iter()
        .filter(|line_segment| {
            line_segment.is_vertical()
                || line_segment.is_horizontal()
                || (diagonal && line_segment.is_diagonal())
        })
        .flat_map(|line_segment| line_segment.trace())
        .collect()
}

fn count_overlap_points(points: &[Point]) -> usize {
    let mut point_count_map: HashMap<&Point, isize> = HashMap::new();

    for point in points {
        point_count_map
            .entry(point)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    point_count_map
        .iter()
        .filter(|(_, &count)| count > 1)
        .count()
}
