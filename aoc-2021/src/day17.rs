use aoc_utils;
use regex::Regex;

type Value = i64;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: Value,
    y: Value,
}

#[derive(Debug, Copy, Clone)]
struct Area {
    left: Value,
    top: Value,
    right: Value,
    bottom: Value,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day17.txt", true).collect();

    let area = parse_area(lines);

    let heights = solve_height_trajectories(&area);

    let height = heights.iter().max().expect("height");
    let count = heights.len();

    println!("{}", height);
    println!("{}", count);
}

fn parse_area(lines: Vec<String>) -> Area {
    let re = r"target area: x=(?P<left>.+)[.]{2}(?P<right>.+), y=(?P<bottom>.+)[.]{2}(?P<top>.+)";
    let re = Regex::new(re).expect("regex");

    let line = lines.iter().next().expect("line");
    let caps = re.captures(&line).expect("captures");

    let left = caps["left"].trim().parse().expect("left");
    let top = caps["top"].trim().parse().expect("top");
    let right = caps["right"].trim().parse().expect("right");
    let bottom = caps["bottom"].trim().parse().expect("bottom");

    Area {
        left,
        top,
        right,
        bottom,
    }
}

fn solve_height_trajectories(area: &Area) -> Vec<Value> {
    let initial_velocities = get_velocities(area);

    initial_velocities
        .iter()
        .filter_map(|velocity| solve_height_trajectory(velocity, area))
        .collect()
}

fn get_velocities(area: &Area) -> Vec<Point> {
    let min_velocity_x = 1;
    let max_velocity_x = area.right;
    let min_velocity_y = area.bottom;
    let max_velocity_y = -area.bottom;

    (min_velocity_x..=max_velocity_x)
        .flat_map(|velocity_x| {
            (min_velocity_y..=max_velocity_y)
                .map(|velocity_y| Point {
                    x: velocity_x,
                    y: velocity_y,
                })
                .collect::<Vec<Point>>()
        })
        .collect()
}

fn solve_height_trajectory(initial_velocity: &Point, area: &Area) -> Option<Value> {
    let mut position = Point { x: 0, y: 0 };
    let mut velocity = initial_velocity.clone();
    let mut height = Value::MIN;

    let drag = 1;
    let gravity = -1;

    loop {
        position.x += velocity.x;
        position.y += velocity.y;

        velocity.x += velocity.x.signum() * -drag;
        velocity.y += gravity;

        height = height.max(position.y);

        let within_area = position.x >= area.left
            && position.x <= area.right
            && position.y >= area.bottom
            && position.y <= area.top;

        let overshoot_area = position.x > area.right || position.y < area.bottom;

        if within_area {
            return Some(height);
        } else if overshoot_area {
            return None;
        }
    }
}
