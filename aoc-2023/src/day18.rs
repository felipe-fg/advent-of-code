use std::iter;

type Plan = Vec<Instruction>;
type Number = i64;
type Vertex = (Number, Number);
type Vertices = Vec<Vertex>;
type Perimeter = Number;
type Polygon = (Vertices, Perimeter);

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    meters: Number,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day18.txt", true).collect();

    let plan = parse_plan(&lines, false);
    let plan_hex = parse_plan(&lines, true);

    let polygon = get_polygon(&plan);
    let area = compute_area(&polygon);

    let polygon_hex = get_polygon(&plan_hex);
    let area_hex = compute_area(&polygon_hex);

    println!("{:?}", area);
    println!("{:?}", area_hex);
}

fn parse_plan(lines: &[String], hex: bool) -> Plan {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();

            if hex {
                let color = parts.nth(2).expect("color");
                let meters = &color[2..7];
                let direction = &color[7..8];

                let meters = Number::from_str_radix(meters, 16).expect("meters");

                let direction = match direction {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => unreachable!(),
                };

                Instruction { direction, meters }
            } else {
                let direction = parts.next().expect("direction");
                let meters = parts.next().expect("meters").parse().expect("meters");

                let direction = match direction {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => unreachable!(),
                };

                Instruction { direction, meters }
            }
        })
        .collect()
}

fn get_polygon(plan: &Plan) -> Polygon {
    let mut vertices = Vertices::new();
    let mut perimeter = 0;

    let mut x = 0;
    let mut y = 0;

    for instruction in plan {
        match instruction.direction {
            Direction::Right => x += instruction.meters,
            Direction::Left => x -= instruction.meters,
            Direction::Up => y += instruction.meters,
            Direction::Down => y -= instruction.meters,
        }

        vertices.push((x, y));
        perimeter += instruction.meters;
    }

    (vertices, perimeter)
}

fn compute_area((vertices, perimeter): &Polygon) -> Number {
    // Pick's theorem
    // A = i + (b / 2) - 1
    // i = A + 1 - (b / 2)
    let approximate_area = shoelace_formula(vertices);
    let boundary_points = perimeter;
    let interior_points = approximate_area + 1 - (boundary_points / 2);

    interior_points + boundary_points
}

fn shoelace_formula(vertices: &Vertices) -> Number {
    let first = vertices.first().expect("first");
    let vertices: Vec<_> = vertices.iter().chain(iter::once(first)).collect();

    let sum: Number = vertices
        .windows(2)
        .map(|window| {
            let (x, y) = window[0];
            let (next_x, next_y) = window[1];

            (x * next_y) - (next_x * y)
        })
        .sum();

    (sum / 2).abs()
}
