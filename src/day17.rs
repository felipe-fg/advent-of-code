use super::utils;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::Range;

type Point = Vec<i128>;

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day17.txt", true).collect();

    let points_three = parse_input(&lines, 3);
    let points_four = parse_input(&lines, 4);

    let count_three = simulate(points_three, 3, 6);
    let count_four = simulate(points_four, 4, 6);

    println!("{}", count_three);
    println!("{}", count_four);
}

fn parse_input(rows: &Vec<String>, dimensions: usize) -> Vec<Point> {
    rows.iter()
        .enumerate()
        .flat_map(|(row, columns)| parse_input_line(row, columns, dimensions))
        .collect()
}

fn parse_input_line(row: usize, columns: &str, dimensions: usize) -> Vec<Point> {
    columns
        .split("")
        .filter(|value| value == &"#" || value == &".")
        .enumerate()
        .filter(|(_, value)| value == &"#")
        .map(move |(column, _)| {
            let mut point = vec![0i128; dimensions];
            point[0] = column as i128;
            point[1] = row as i128;

            point
        })
        .collect()
}

fn simulate(points: Vec<Point>, dimensions: usize, cycles: usize) -> usize {
    let neighbors = get_neighbors(dimensions);

    let mut cubes = HashSet::new();
    let mut min = vec![-1; dimensions];
    let mut max = vec![1; dimensions];

    for point in points {
        update_bounds(&point, &mut min, &mut max);
        set_active(&mut cubes, point, true);
    }

    for _ in 0..cycles {
        let mut next_cubes = cubes.clone();

        let ranges = (0..dimensions)
            .map(|index| min[index]..(max[index] + 1))
            .collect();

        for point in cartesian_product(ranges) {
            let count = get_neighbors_count(&cubes, &neighbors, &point);
            let active = get_active(&cubes, &point);

            if active && !(count == 2 || count == 3) {
                set_active(&mut next_cubes, point, false);
            } else if !active && count == 3 {
                update_bounds(&point, &mut min, &mut max);
                set_active(&mut next_cubes, point, true);
            }
        }

        cubes = next_cubes;
    }

    cubes.len()
}

fn get_neighbors(dimensions: usize) -> Vec<Point> {
    let ranges = (0..dimensions).map(|_| (-1..2)).collect();

    cartesian_product(ranges)
        .into_iter()
        .filter(|values| !values.iter().all(|value| value == &0))
        .collect()
}

fn get_neighbors_count(cubes: &HashSet<Point>, neighbors: &Vec<Point>, point: &Point) -> usize {
    neighbors
        .iter()
        .filter(|neighbor| {
            let neighbor_point = point
                .iter()
                .enumerate()
                .map(|(index, value)| value + neighbor[index])
                .collect();

            get_active(cubes, &neighbor_point)
        })
        .count()
}

fn get_active(cubes: &HashSet<Point>, point: &Point) -> bool {
    cubes.contains(point)
}

fn set_active(cubes: &mut HashSet<Point>, point: Point, active: bool) {
    if active {
        cubes.insert(point);
    } else {
        cubes.remove(&point);
    }
}

fn update_bounds(point: &Point, min: &mut Point, max: &mut Point) {
    *min = min
        .iter()
        .enumerate()
        .map(|(index, value)| *value.min(&(point[index] - 1)))
        .collect();

    *max = max
        .iter()
        .enumerate()
        .map(|(index, value)| *value.max(&(point[index] + 1)))
        .collect();
}

fn cartesian_product(mut ranges: Vec<Range<i128>>) -> Vec<Vec<i128>> {
    fn nested(mut ranges: Vec<Range<i128>>, items: Vec<Vec<i128>>) -> Vec<Vec<i128>> {
        let items = ranges
            .remove(0)
            .cartesian_product(items.into_iter())
            .map(|(item, vec)| {
                let mut vec = vec.clone();
                vec.push(item);
                vec
            })
            .collect();
        if ranges.is_empty() {
            items
        } else {
            nested(ranges, items)
        }
    }

    let items = ranges
        .remove(0)
        .cartesian_product(ranges.remove(0))
        .map(|(left, right)| vec![left, right])
        .collect();

    if ranges.is_empty() {
        items
    } else {
        nested(ranges, items)
    }
}
