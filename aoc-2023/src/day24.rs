use itertools::Itertools;
use num::rational::Ratio;
use num::Zero;
use rayon::prelude::*;
use regex::Regex;

type Number = Ratio<i128>;
type Objects = Vec<Object>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: Number,
    y: Number,
}

impl Point {
    fn new(x: i128, y: i128) -> Self {
        Self {
            x: Number::from_integer(x),
            y: Number::from_integer(y),
        }
    }

    fn is_inside_area(&self, min: &Point, max: &Point) -> bool {
        let is_inside_x = self.x >= min.x && self.x <= max.x;
        let is_inside_y = self.y >= min.y && self.y <= max.y;

        is_inside_x && is_inside_y
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn intersect(&self, other: &Self) -> Option<Point> {
        let x1 = self.a.x;
        let y1 = self.a.y;
        let x2 = self.b.x;
        let y2 = self.b.y;

        let x3 = other.a.x;
        let y3 = other.a.y;
        let x4 = other.b.x;
        let y4 = other.b.y;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denominator.is_zero() {
            return None;
        }

        let x_numerator = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let y_numerator = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        let x = (x_numerator) / (denominator);
        let y = (y_numerator) / (denominator);

        Some(Point { x, y })
    }

    fn is_before_a(&self, c: &Point) -> bool {
        let ac_x = c.x - self.a.x;
        let ac_y = c.y - self.a.y;

        let ab_x = self.b.x - self.a.x;
        let ab_y = self.b.y - self.a.y;

        let dot_product = (ac_x * ab_x) + (ac_y * ab_y);

        dot_product <= Number::zero()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vector {
    x: Number,
    y: Number,
    z: Number,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Object {
    position: Vector,
    velocity: Vector,
}

impl Object {
    fn intersect_xy(&self, other: &Self) -> Option<Point> {
        let line_self = self.line_xy();
        let line_other = other.line_xy();

        let intersection = line_self.intersect(&line_other);

        intersection
            .filter(|intersection| !line_self.is_before_a(intersection))
            .filter(|intersection| !line_other.is_before_a(intersection))
    }

    fn intersect_xz(&self, other: &Self) -> Option<Point> {
        let line_self = self.line_xz();
        let line_other = other.line_xz();

        let intersection = line_self.intersect(&line_other);

        intersection
            .filter(|intersection| !line_self.is_before_a(intersection))
            .filter(|intersection| !line_other.is_before_a(intersection))
    }

    fn line_xy(&self) -> Line {
        Line {
            a: Point {
                x: self.position.x,
                y: self.position.y,
            },
            b: Point {
                x: self.position.x + self.velocity.x,
                y: self.position.y + self.velocity.y,
            },
        }
    }

    fn line_xz(&self) -> Line {
        Line {
            a: Point {
                x: self.position.x,
                y: self.position.z,
            },
            b: Point {
                x: self.position.x + self.velocity.x,
                y: self.position.z + self.velocity.z,
            },
        }
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day24.txt", true).collect();

    let hailstones = parse_hailstones(&lines);
    let min = Point::new(200000000000000, 200000000000000);
    let max = Point::new(400000000000000, 400000000000000);

    let count = count_hailstone_intersections(&hailstones, min, max);

    let rock = estimate_rock_intersection(&hailstones, 300);
    let sum = rock.position.x + rock.position.y + rock.position.z;

    println!("{:?}", count);
    println!("{}", sum);
}

fn parse_hailstones(lines: &[String]) -> Objects {
    let re = r"(?P<px>-?\d+),\s+(?P<py>-?\d+),\s+(?P<pz>-?\d+)\s+@\s+(?P<vx>-?\d+),\s+(?P<vy>-?\d+),\s+(?P<vz>-?\d+)";
    let re = Regex::new(re).expect("regex");

    lines
        .iter()
        .map(|line| {
            let caps = re.captures(line).expect("captures");

            let position = Vector {
                x: caps["px"].parse().expect("px"),
                y: caps["py"].parse().expect("py"),
                z: caps["pz"].parse().expect("pz"),
            };

            let velocity = Vector {
                x: caps["vx"].parse().expect("vx"),
                y: caps["vy"].parse().expect("vy"),
                z: caps["vz"].parse().expect("vz"),
            };

            Object { position, velocity }
        })
        .collect()
}

fn count_hailstone_intersections(hailstones: &Objects, min: Point, max: Point) -> usize {
    hailstones
        .iter()
        .combinations(2)
        .filter_map(|hailstones| {
            hailstones[0]
                .intersect_xy(hailstones[1])
                .filter(|intersection| intersection.is_inside_area(&min, &max))
        })
        .count()
}

fn estimate_rock_intersection(hailstones: &Objects, range: i128) -> Object {
    let (rock_velocity_xy, rock_position_xy) = (-range..=range)
        .into_par_iter()
        .find_map_any(|x| {
            (-range..=range).find_map(|y| {
                let rock_velocity_xy = (Number::from_integer(x), Number::from_integer(y));

                let rock_position_xy = estimate_rock_position_xy(hailstones, rock_velocity_xy);

                rock_position_xy.map(|rock_position_xy| (rock_velocity_xy, rock_position_xy))
            })
        })
        .expect("rock xy");

    let (rock_velocity_xz, rock_position_xz) = (-range..=range)
        .into_par_iter()
        .find_map_any(|z| {
            let rock_velocity_xz = (rock_velocity_xy.0, Number::from_integer(z));

            let rock_position_xz = estimate_rock_position_xz(hailstones, rock_velocity_xz);

            rock_position_xz.map(|rock_position_xz| (rock_velocity_xz, rock_position_xz))
        })
        .expect("rock xz");

    let rock_position = Vector {
        x: rock_position_xy.x,
        y: rock_position_xy.y,
        z: rock_position_xz.y,
    };

    let rock_velocity = Vector {
        x: rock_velocity_xy.0,
        y: rock_velocity_xy.1,
        z: rock_velocity_xz.1,
    };

    Object {
        position: rock_position,
        velocity: rock_velocity,
    }
}

fn estimate_rock_position_xy(
    hailstones: &Objects,
    (rock_velocity_x, rock_velocity_y): (Number, Number),
) -> Option<Point> {
    let hailstones: Objects = hailstones
        .iter()
        .map(|hailstone| Object {
            position: hailstone.position,
            velocity: Vector {
                x: hailstone.velocity.x - rock_velocity_x,
                y: hailstone.velocity.y - rock_velocity_y,
                z: hailstone.velocity.z,
            },
        })
        .collect();

    let mut iter = hailstones.into_iter();
    let mut intersection = None;

    let first = iter.next().expect("first");

    for hailstone in iter {
        let intersection_first = first.intersect_xy(&hailstone);

        match intersection_first {
            Some(intersection_first) => match intersection {
                None => {
                    intersection = Some(intersection_first);

                    continue;
                }
                Some(intersection) => {
                    if intersection != intersection_first {
                        return None;
                    }
                }
            },
            None => return None,
        }
    }

    intersection
}

fn estimate_rock_position_xz(
    hailstones: &Objects,
    (rock_velocity_x, rock_velocity_z): (Number, Number),
) -> Option<Point> {
    let hailstones: Objects = hailstones
        .iter()
        .map(|hailstone| Object {
            position: hailstone.position,
            velocity: Vector {
                x: hailstone.velocity.x - rock_velocity_x,
                y: hailstone.velocity.y,
                z: hailstone.velocity.z - rock_velocity_z,
            },
        })
        .collect();

    let mut iter = hailstones.into_iter();
    let mut intersection = None;

    let first = iter.next().expect("first");

    for hailstone in iter {
        let intersection_first = first.intersect_xz(&hailstone);

        match intersection_first {
            Some(intersection_first) => match intersection {
                None => {
                    intersection = Some(intersection_first);

                    continue;
                }
                Some(intersection) => {
                    if intersection != intersection_first {
                        return None;
                    }
                }
            },
            None => return None,
        }
    }

    intersection
}
