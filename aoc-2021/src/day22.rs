use aoc_utils;
use regex::Regex;

type Value = i128;

const INITIALIZATION: Value = 50;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cuboid {
    left: Value,
    bottom: Value,
    back: Value,
    right: Value,
    top: Value,
    front: Value,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    status: bool,
    cuboid: Cuboid,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day22.txt", true).collect();

    let steps = parse_steps(lines);

    let volume_partial = reboot_reactor(&steps, false);
    let volume_full = reboot_reactor(&steps, true);

    println!("{}", volume_partial);
    println!("{}", volume_full);
}

fn parse_steps(lines: Vec<String>) -> Vec<State> {
    lines.iter().map(|line| parse_step(&line)).collect()
}

fn parse_step(line: &str) -> State {
    let regex = r"(?P<status>\w+) x=(?P<left>.+)[.]{2}(?P<right>.+),y=(?P<bottom>.+)[.]{2}(?P<top>.+),z=(?P<back>.+)[.]{2}(?P<front>.+)";
    let re = Regex::new(regex).expect("regex");

    let caps = re.captures(&line).expect("captures");

    let status = caps["status"].trim() == "on";

    let left = caps["left"].trim().to_string().parse().expect("number");
    let bottom = caps["bottom"].trim().to_string().parse().expect("number");
    let back = caps["back"].trim().to_string().parse().expect("number");
    let right = caps["right"].trim().to_string().parse().expect("number");
    let top = caps["top"].trim().to_string().parse().expect("number");
    let front = caps["front"].trim().to_string().parse().expect("number");

    let cuboid = Cuboid {
        left,
        bottom,
        back,
        right,
        top,
        front,
    };

    State { status, cuboid }
}

fn reboot_reactor(steps: &[State], full: bool) -> Value {
    let steps: Vec<State> = steps
        .iter()
        .filter(|step| full || initialization_step(step))
        .map(|&step| step)
        .collect();

    let mut states: Vec<State> = vec![];

    for State { status, cuboid } in steps {
        let mut next_states = vec![];

        for &state in &states {
            if let Some(next_cuboid) = intersect_cuboid(&cuboid, &state.cuboid) {
                let next_status = match (status, state.status) {
                    (true, true) => false,
                    (false, false) => true,
                    (false, true) => false,
                    (true, false) => true,
                };

                let next_state = State {
                    status: next_status,
                    cuboid: next_cuboid,
                };

                next_states.push(next_state);
            }
        }

        if status {
            next_states.push(State { status, cuboid });
        }

        states.append(&mut next_states);
    }

    volume_reactor(&states)
}

fn initialization_step(step: &State) -> bool {
    let x = step.cuboid.left >= -INITIALIZATION && step.cuboid.right <= INITIALIZATION;
    let y = step.cuboid.bottom >= -INITIALIZATION && step.cuboid.top <= INITIALIZATION;
    let z = step.cuboid.back >= -INITIALIZATION && step.cuboid.front <= INITIALIZATION;

    x && y && z
}

fn volume_reactor(states: &[State]) -> Value {
    states
        .iter()
        .map(|state| match state.status {
            true => volume_cuboid(&state.cuboid),
            false => -volume_cuboid(&state.cuboid),
        })
        .sum()
}

fn intersect_cuboid(a: &Cuboid, b: &Cuboid) -> Option<Cuboid> {
    let x = intersect_line(a.left, a.right, b.left, b.right);
    let y = intersect_line(a.bottom, a.top, b.bottom, b.top);
    let z = intersect_line(a.back, a.front, b.back, b.front);

    let intersection = x.zip(y).zip(z);

    intersection.map(|(((left, right), (bottom, top)), (back, front))| Cuboid {
        left,
        bottom,
        back,
        right,
        top,
        front,
    })
}

fn intersect_line(
    min_a: Value,
    max_a: Value,
    min_b: Value,
    max_b: Value,
) -> Option<(Value, Value)> {
    if min_a <= max_b && max_a >= min_b {
        let min = min_a.max(min_b);
        let max = max_a.min(max_b);

        let intersection = (min, max);

        Some(intersection)
    } else {
        None
    }
}

fn volume_cuboid(cuboid: &Cuboid) -> Value {
    let x = cuboid.right - cuboid.left + 1;
    let y = cuboid.top - cuboid.bottom + 1;
    let z = cuboid.front - cuboid.back + 1;

    x * y * z
}
