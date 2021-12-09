use aoc_utils;
use colored::*;
use itertools::Itertools;
use rayon::prelude::*;

type X = isize;
type Y = isize;
type Point = (X, Y);

type Height = usize;
type Heightmap = Vec<Vec<Height>>;

type Data = (Point, Height);

const HEIGHT_BLOCKED: usize = 9;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day09.txt", true).collect();

    let heightmap = parse_heightmap(lines);

    let low_points = find_low_points(&heightmap);

    let basins = find_basins(&heightmap, &low_points);

    let risk_levels = sum_risk_levels(&low_points);

    let largest_basins = size_largest_basins(&basins, 3);

    draw_heightmap(&heightmap, &low_points, &basins);

    println!("{}", risk_levels);
    println!("{}", largest_basins);
}

fn parse_heightmap(lines: Vec<String>) -> Heightmap {
    lines
        .par_iter()
        .map(|line| {
            line.split("")
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .map(|value| value.parse().expect("number"))
                .collect()
        })
        .collect()
}

fn sum_risk_levels(low_points: &[Data]) -> usize {
    low_points.par_iter().map(|&(_, height)| height + 1).sum()
}

fn size_largest_basins(basins: &[Vec<Data>], count: usize) -> usize {
    basins
        .iter()
        .take(count)
        .map(|basin| basin.len())
        .reduce(|a, b| a * b)
        .expect("largest basins")
}

fn find_low_points(heightmap: &Heightmap) -> Vec<Data> {
    heightmap
        .par_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.par_iter()
                .enumerate()
                .map(|(x, &height)| ((x as isize, y as isize), height))
                .filter(|(point, _)| is_low_point(heightmap, point))
                .collect::<Vec<Data>>()
        })
        .collect()
}

fn is_low_point(heightmap: &Heightmap, &(x, y): &Point) -> bool {
    let height = get_height(heightmap, &(x, y));

    let top = get_height(heightmap, &(x, y - 1));
    let down = get_height(heightmap, &(x, y + 1));
    let left = get_height(heightmap, &(x - 1, y));
    let right = get_height(heightmap, &(x + 1, y));

    height < top.min(down).min(left).min(right)
}

fn find_basins(heightmap: &Heightmap, low_points: &[Data]) -> Vec<Vec<Data>> {
    let basins: Vec<Vec<Data>> = low_points
        .par_iter()
        .map(|low_point| find_basin(heightmap, low_point))
        .collect();

    basins
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()))
        .collect()
}

fn find_basin(heightmap: &Heightmap, &low_point: &Data) -> Vec<Data> {
    let mut basin: Vec<Data> = vec![];
    let mut remaining: Vec<Data> = vec![low_point];

    while let Some((point, height)) = remaining.pop() {
        basin.push((point, height));

        let neighbors = get_neighbors(heightmap, &point);

        for neighbor in neighbors {
            if !basin.contains(&neighbor) && !remaining.contains(&neighbor) {
                remaining.push(neighbor);
            }
        }
    }

    basin
}

fn get_neighbors(heightmap: &Heightmap, &(x, y): &Point) -> Vec<Data> {
    vec![(0, -1), (0, 1), (-1, 0), (1, 0)]
        .par_iter()
        .map(|(direction_x, direction_y)| {
            let point = ((x + direction_x), (y + direction_y));
            let height = get_height(heightmap, &point);

            (point, height)
        })
        .filter(|&(_, height)| height < HEIGHT_BLOCKED)
        .collect()
}

fn get_height(heightmap: &Heightmap, &(x, y): &Point) -> Height {
    heightmap
        .get(y as usize)
        .map(|row| row.get(x as usize).map(|&height| height))
        .flatten()
        .unwrap_or(HEIGHT_BLOCKED)
}

fn draw_heightmap(heightmap: &Heightmap, low_points: &[Data], basins: &[Vec<Data>]) -> () {
    let first = basins.get(0).expect("first");
    let second = basins.get(1).expect("second");
    let third = basins.get(2).expect("third");

    for (y, row) in heightmap.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            let data = ((x as isize, y as isize), height);

            if low_points.contains(&data) {
                print!("{}", "██".bright_white());
            } else if first.contains(&data) {
                print!("{}", "██".red());
            } else if second.contains(&data) {
                print!("{}", "██".green());
            } else if third.contains(&data) {
                print!("{}", "██".blue());
            } else if height == HEIGHT_BLOCKED {
                print!("{}", "██".black());
            } else {
                print!("{}", "██".white());
            }
        }

        println!();
    }
}
