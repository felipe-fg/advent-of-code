use std::collections::{HashMap, HashSet};

use colored::*;

type Position = (usize, usize);
type Positions = HashSet<Position>;
type Distance = usize;
type Path = HashMap<Position, Distance>;
type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day10.txt", true).collect();

    let grid = parse_grid(&lines);

    let path = find_path(&grid);
    let enclosed = find_enclosed(&grid, &path);

    draw_grid(&grid, &path, &enclosed);

    println!("{:?}", path.len() / 2);
    println!("{:?}", enclosed.len());
}

fn parse_grid(lines: &[String]) -> Grid {
    lines
        .iter()
        .map(|line| line.chars().map(parse_tile).collect())
        .collect()
}

fn parse_tile(char: char) -> Tile {
    match char {
        '|' => Tile::NorthSouth,
        '-' => Tile::EastWest,
        'L' => Tile::NorthEast,
        'J' => Tile::NorthWest,
        '7' => Tile::SouthWest,
        'F' => Tile::SouthEast,
        '.' => Tile::Ground,
        'S' => Tile::Start,
        _ => Tile::Ground,
    }
}

fn find_path(grid: &Grid) -> Path {
    let start = find_start(grid).expect("start");

    let mut path = Path::new();
    let mut entry = Some((start, path.len()));

    while let Some((position, distance)) = entry {
        path.insert(position, distance);

        let next_entry =
            find_next_position(grid, &path, position).map(|(position, _)| (position, path.len()));

        entry = next_entry
    }

    path
}

fn find_start(grid: &Grid) -> Option<Position> {
    grid.iter()
        .enumerate()
        .flat_map(|(row_index, columns)| {
            columns
                .iter()
                .enumerate()
                .map(move |(column_index, tile)| (column_index, row_index, tile))
        })
        .find(|(_, _, tile)| tile == &&Tile::Start)
        .map(|(column_index, row_index, _)| (column_index, row_index))
}

fn find_next_position(grid: &Grid, path: &Path, (x, y): Position) -> Option<(Position, Tile)> {
    let current = ((x, y), get_tile(grid, (x, y)).expect("current"));

    let neighbors = vec![
        (x as isize, y as isize - 1),
        (x as isize, y as isize + 1),
        (x as isize - 1, y as isize),
        (x as isize + 1, y as isize),
    ]
    .into_iter()
    .filter(|(x, y)| x >= &0 && y >= &0)
    .map(|(x, y)| (x as usize, y as usize));

    neighbors
        .into_iter()
        .filter(|position| !path.contains_key(position))
        .filter_map(|position| get_tile(grid, position).map(|tile| (position, tile)))
        .filter(|(_, tile)| tile != &Tile::Start)
        .find(|next| contains_path_connection(&current, next))
}

fn contains_path_connection(
    ((current_x, current_y), current_tile): &(Position, Tile),
    ((next_x, next_y), next_tile): &(Position, Tile),
) -> bool {
    let north_connection = matches!(
        current_tile,
        Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast | Tile::Start
    ) && matches!(
        next_tile,
        Tile::SouthEast | Tile::SouthWest | Tile::NorthSouth
    ) && current_y > next_y;

    let south_connection = matches!(
        current_tile,
        Tile::SouthEast | Tile::SouthWest | Tile::NorthSouth | Tile::Start
    ) && matches!(
        next_tile,
        Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast
    ) && current_y < next_y;

    let west_connection = matches!(
        current_tile,
        Tile::NorthWest | Tile::SouthWest | Tile::EastWest | Tile::Start
    ) && matches!(
        next_tile,
        Tile::NorthEast | Tile::SouthEast | Tile::EastWest
    ) && current_x > next_x;

    let east_connection = matches!(
        current_tile,
        Tile::NorthEast | Tile::SouthEast | Tile::EastWest | Tile::Start
    ) && matches!(
        next_tile,
        Tile::NorthWest | Tile::SouthWest | Tile::EastWest
    ) && current_x < next_x;

    north_connection || south_connection || west_connection || east_connection
}

fn find_enclosed(grid: &Grid, path: &Path) -> Positions {
    let mut positions = Positions::new();

    for (row_index, columns) in grid.iter().enumerate() {
        let mut crossings: isize = 0;

        for (column_index, _) in columns.iter().enumerate() {
            let current_position = (column_index, row_index);
            let current_tile = get_tile(grid, current_position);
            let current_distance = path.get(&current_position);
            let current_is_path = path.contains_key(&current_position);

            let bottom_position = (column_index, row_index + 1);
            let bottom_tile = get_tile(grid, bottom_position);
            let bottom_distance = path.get(&bottom_position);

            if let (
                Some(current_tile),
                Some(current_distance),
                Some(bottom_tile),
                Some(bottom_distance),
            ) = (current_tile, current_distance, bottom_tile, bottom_distance)
            {
                let current = (current_position, current_tile);
                let bottom = (bottom_position, bottom_tile);

                let contains_connection = contains_path_connection(&current, &bottom);

                if contains_connection && current_is_path {
                    let value = if current_distance > bottom_distance {
                        1
                    } else {
                        -1
                    };

                    crossings += value;
                }
            }

            if !current_is_path {
                let is_enclosed = crossings != 0;

                if is_enclosed {
                    positions.insert((column_index, row_index));
                }
            }
        }
    }

    positions
}

fn get_tile(grid: &Grid, (x, y): Position) -> Option<Tile> {
    grid.get(y).and_then(|columns| columns.get(x)).cloned()
}

fn draw_grid(grid: &Grid, path: &Path, enclosed: &Positions) {
    for (row_index, columns) in grid.iter().enumerate() {
        for (column_index, tile) in columns.iter().enumerate() {
            let position = (column_index, row_index);
            let is_path = path.contains_key(&position);
            let is_enclosed = enclosed.contains(&position);

            let tile = match tile {
                Tile::NorthSouth => "|".white(),
                Tile::EastWest => "─".white(),
                Tile::NorthEast => "└".white(),
                Tile::NorthWest => "┘".white(),
                Tile::SouthWest => "┐".white(),
                Tile::SouthEast => "┌".white(),
                Tile::Ground => " ".black(),
                Tile::Start => "+".white(),
            };

            let tile = if is_path {
                tile.red()
            } else if is_enclosed {
                tile.blue()
            } else {
                tile
            };

            print!("{}", tile.on_black());
        }

        println!();
    }
}
