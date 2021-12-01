use aoc_utils;
use colored::*;
use rayon::prelude::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Clone)]
struct Tile {
    id: u128,
    data: Vec<Vec<String>>,
}

impl Tile {
    fn rows(&self) -> usize {
        self.data.len()
    }

    fn columns(&self) -> usize {
        self.data.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn rotate_clockwise(&self) -> Self {
        let rows = self.rows();
        let columns = self.columns();

        let new_data = (0..columns)
            .map(|j| {
                (0..rows)
                    .map(|i| self.data[rows - 1 - i][j].to_string())
                    .collect()
            })
            .collect();

        Self {
            id: self.id,
            data: new_data,
        }
    }

    fn flip_horizontal(&self) -> Self {
        let rows = self.rows();
        let columns = self.columns();

        let new_data = (0..rows)
            .map(|i| {
                (0..columns)
                    .map(|j| self.data[i][columns - 1 - j].to_string())
                    .collect()
            })
            .collect();

        Self {
            id: self.id,
            data: new_data,
        }
    }

    fn flip_vertical(&self) -> Self {
        let rows = self.rows();
        let columns = self.columns();

        let new_data = (0..rows)
            .map(|i| {
                (0..columns)
                    .map(|j| self.data[rows - 1 - i][j].to_string())
                    .collect()
            })
            .collect();

        Self {
            id: self.id,
            data: new_data,
        }
    }

    fn top(&self) -> String {
        self.data
            .first()
            .map(|row| row.par_iter().map(|value| value.to_string()).collect())
            .unwrap_or(String::from(""))
    }

    fn left(&self) -> String {
        self.data
            .par_iter()
            .map(|row| {
                row.first()
                    .map(|value| value.to_string())
                    .unwrap_or(String::from(""))
            })
            .collect()
    }

    fn bottom(&self) -> String {
        self.data
            .last()
            .map(|row| row.par_iter().map(|value| value.to_string()).collect())
            .unwrap_or(String::from(""))
    }

    fn right(&self) -> String {
        self.data
            .par_iter()
            .map(|row| {
                row.last()
                    .map(|value| value.to_string())
                    .unwrap_or(String::from(""))
            })
            .collect()
    }

    fn combinations(&self) -> Vec<Self> {
        let original = self.clone();
        let original_fh = original.flip_horizontal();
        let original_fv = original.flip_vertical();

        let rotated_one = original.rotate_clockwise();
        let rotated_one_fh = rotated_one.flip_horizontal();

        let rotated_two = rotated_one.rotate_clockwise();

        let rotated_three = rotated_two.rotate_clockwise();
        let rotated_three_fh = rotated_three.flip_horizontal();

        vec![
            original,
            original_fh,
            original_fv,
            rotated_one,
            rotated_one_fh,
            rotated_two,
            rotated_three,
            rotated_three_fh,
        ]
    }

    fn corner_match(&self, other: &Self, direction: Direction) -> bool {
        match direction {
            Direction::Top => self.top() == other.bottom(),
            Direction::Left => self.left() == other.right(),
            Direction::Bottom => self.bottom() == other.top(),
            Direction::Right => self.right() == other.left(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Tile {}:\n", self.id)?;

        for row in &self.data {
            for value in row {
                let value = match &value[..] {
                    "O" => "██".red(),
                    "#" => "██".blue(),
                    "." => "██".cyan(),
                    _ => "".white(),
                };

                write!(f, "{}", value)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

struct Monster {
    data: Vec<Vec<String>>,
    rows: usize,
    columns: usize,
    blocks: usize,
}

impl Monster {
    fn new() -> Monster {
        let image = vec![
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ];

        let data: Vec<Vec<String>> = image
            .iter()
            .map(|row| row.chars().map(|value| value.to_string()).collect())
            .collect();

        let rows = data.len();
        let columns = data[0].len();

        let blocks: usize = data
            .iter()
            .map(|row| row.iter().filter(|value| value == &"#").count())
            .sum();

        Monster {
            data,
            rows,
            columns,
            blocks,
        }
    }

    fn find(&self, tile: &Tile, row: usize, column: usize) -> bool {
        let mut match_blocks = 0;

        for i in row..row + self.rows {
            for j in column..column + self.columns {
                let value_match = tile
                    .data
                    .get(i)
                    .map(|row| row.get(j))
                    .flatten()
                    .filter(|value| value == &"#");

                if self.data[i - row][j - column] == "#" && value_match.is_some() {
                    match_blocks += 1;
                }
            }
        }

        match_blocks == self.blocks
    }

    fn replace(&self, tile: &mut Tile, row: usize, column: usize) {
        for i in row..row + self.rows {
            for j in column..column + self.columns {
                let value_match = tile
                    .data
                    .get(i)
                    .map(|row| row.get(j))
                    .flatten()
                    .filter(|value| value == &"#");

                if self.data[i - row][j - column] == "#" && value_match.is_some() {
                    tile.data[i][j] = String::from("O");
                }
            }
        }
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day20.txt", false).collect();

    let tiles = parse_tiles(&lines);

    let combinations = build_combinations(&tiles);

    let jigsaw = build_jigsaw(&combinations);

    let image = build_image(&jigsaw);

    let oriented = find_oriented_monsters(&image);

    let blocks = count_image_blocks(&oriented);

    println!("{}", oriented);
    println!("{}", blocks);
}

fn parse_tiles(lines: &[String]) -> Vec<Tile> {
    let mut tiles = vec![];
    let mut iter = lines.iter().enumerate();
    let mut previous_length = 0;

    while let Some((index, line)) = iter.next() {
        if line.len() >= 4 && &line[..4] == "Tile" {
            let length = lines[index..]
                .iter()
                .position(|line| line.is_empty())
                .unwrap_or(previous_length);

            previous_length = length;

            let tile = parse_tile(&lines[index..index + length]);

            tiles.push(tile);

            iter.nth(length - 1);
        }
    }

    tiles
}

fn parse_tile(lines: &[String]) -> Tile {
    let id = lines[0]
        .replace("Tile ", "")
        .replace(":", "")
        .parse()
        .expect("id");

    let data = lines[1..]
        .par_iter()
        .map(|line| {
            line.split("")
                .filter(|value| value == &"." || value == &"#")
                .map(|value| value.to_string())
                .collect()
        })
        .collect();

    Tile { id, data }
}

fn build_combinations(tiles: &[Tile]) -> Vec<Tile> {
    tiles
        .par_iter()
        .flat_map(|tile| tile.combinations())
        .collect()
}

fn find_top_left(combinations: &[Tile]) -> Tile {
    combinations
        .par_iter()
        .find_any(|combination| {
            let top = find_next(&combinations, &combination, Direction::Top);
            let left = find_next(&combinations, &combination, Direction::Left);
            let bottom = find_next(&combinations, &combination, Direction::Bottom);
            let right = find_next(&combinations, &combination, Direction::Right);

            top.is_none() && left.is_none() && bottom.is_some() && right.is_some()
        })
        .expect("top left")
        .clone()
}

fn find_next(combinations: &[Tile], tile: &Tile, direction: Direction) -> Option<Tile> {
    combinations
        .par_iter()
        .filter(|other| other.id != tile.id)
        .find_any(|other| tile.corner_match(other, direction))
        .map(|tile| tile.clone())
}

fn build_jigsaw(combinations: &[Tile]) -> Vec<Vec<Tile>> {
    let top_left = find_top_left(&combinations);

    let mut jigsaw = vec![];
    let mut current_row = vec![top_left];

    loop {
        let current = current_row.last().expect("last");
        let right = find_next(&combinations, &current, Direction::Right);

        if let Some(right) = right {
            current_row.push(right);
        } else {
            jigsaw.push(current_row);

            let current = jigsaw.last().expect("last").first().expect("first");
            let bottom = find_next(&combinations, &current, Direction::Bottom);

            if let Some(bottom) = bottom {
                current_row = vec![bottom];
            } else {
                break;
            }
        }
    }

    jigsaw
}

fn build_image(jigsaw: &Vec<Vec<Tile>>) -> Tile {
    let id = build_image_id(jigsaw);

    let data = jigsaw
        .iter()
        .flat_map(|row_tiles| {
            let internal_rows = row_tiles[0].rows();
            let internal_columns = row_tiles[0].columns();

            (1..internal_rows - 1)
                .map(|internal_row| {
                    row_tiles
                        .iter()
                        .flat_map(|row_tile| {
                            row_tile.data[internal_row][1..internal_columns - 1]
                                .iter()
                                .map(|internal_column| internal_column.to_string())
                        })
                        .collect()
                })
                .collect::<Vec<Vec<String>>>()
        })
        .collect();

    Tile { id, data }
}

fn build_image_id(jigsaw: &Vec<Vec<Tile>>) -> u128 {
    let top_left = jigsaw.first().expect("top").first().expect("left");
    let top_right = jigsaw.first().expect("top").last().expect("right");
    let bottom_left = jigsaw.last().expect("bottom").first().expect("left");
    let bottom_right = jigsaw.last().expect("bottom").last().expect("right");

    top_left.id * top_right.id * bottom_left.id * bottom_right.id
}

fn find_oriented_monsters(image: &Tile) -> Tile {
    let monster = Monster::new();

    let mut oriented = image
        .combinations()
        .into_iter()
        .find(|combination| {
            (0..combination.rows())
                .find(|i| {
                    (0..combination.columns())
                        .find(|j| monster.find(combination, *i, *j))
                        .is_some()
                })
                .is_some()
        })
        .expect("oriented");

    for i in 0..image.rows() {
        for j in 0..image.columns() {
            if monster.find(&oriented, i, j) {
                monster.replace(&mut oriented, i, j)
            }
        }
    }

    oriented
}

fn count_image_blocks(image: &Tile) -> usize {
    image
        .data
        .iter()
        .map(|row| row.iter().filter(|value| value == &"#").count())
        .sum()
}
