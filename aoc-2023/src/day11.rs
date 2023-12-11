use itertools::Itertools;

type Number = isize;
type Pixel = char;
type Image = Vec<Vec<Pixel>>;
type Position = (Number, Number);
type Positions = Vec<Position>;
type EmptySpace = Vec<bool>;

const EMPTY: char = '.';
const GALAXY: char = '#';

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day11.txt", true).collect();

    let image = parse_image(&lines);
    let empty_space = get_empty_space(&image);

    let galaxies = find_galaxies(&image);
    let distance_two = find_shortest_path(&galaxies, &empty_space, 2);
    let distance_million = find_shortest_path(&galaxies, &empty_space, 1000000);

    println!("{:?}", distance_two);
    println!("{:?}", distance_million);
}

fn parse_image(lines: &[String]) -> Image {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn get_empty_space(image: &Image) -> (EmptySpace, EmptySpace) {
    let empty_rows = get_empty_rows(image);

    let image = transpose_image(image);

    let empty_columns = get_empty_rows(&image);

    (empty_columns, empty_rows)
}

fn get_empty_rows(image: &Image) -> EmptySpace {
    image
        .iter()
        .map(|row| row.iter().all(|pixel| pixel == &EMPTY))
        .collect()
}

fn transpose_image(image: &Image) -> Image {
    let rows = image.len();
    let columns = image.first().map(|row| row.len()).unwrap_or_default();

    (0..columns)
        .map(|column| (0..rows).map(|row| image[row][column]).collect())
        .collect()
}

fn find_galaxies(image: &Image) -> Positions {
    image
        .iter()
        .enumerate()
        .flat_map(|(row_index, columns)| {
            columns
                .iter()
                .enumerate()
                .map(move |(column_index, pixel)| (column_index, row_index, pixel))
        })
        .filter(|(_, _, pixel)| pixel == &&GALAXY)
        .map(|(column_index, row_index, _)| (column_index as isize, row_index as isize))
        .collect()
}

fn find_shortest_path(
    galaxies: &Positions,
    empty_space: &(EmptySpace, EmptySpace),
    expand_size: Number,
) -> Number {
    galaxies
        .iter()
        .combinations(2)
        .map(|galaxies| {
            expanded_manhattan_distance(galaxies[0], galaxies[1], empty_space, expand_size)
        })
        .sum()
}

fn expanded_manhattan_distance(
    (a_x, a_y): &Position,
    (b_x, b_y): &Position,
    (empty_columns, empty_rows): &(EmptySpace, EmptySpace),
    expand_size: Number,
) -> Number {
    let distance_x = (b_x - a_x).abs();
    let distance_y = (b_y - a_y).abs();

    let range_columns = *b_x.min(a_x) as usize..=*b_x.max(a_x) as usize;
    let range_rows = *b_y.min(a_y) as usize..=*b_y.max(a_y) as usize;

    let count_empty_columns = empty_columns[range_columns]
        .iter()
        .filter(|empty| **empty)
        .count() as isize;

    let count_empty_rows = empty_rows[range_rows]
        .iter()
        .filter(|empty| **empty)
        .count() as isize;

    let distance_x = distance_x - count_empty_columns + (count_empty_columns * expand_size);
    let distance_y = distance_y - count_empty_rows + (count_empty_rows * expand_size);

    distance_x + distance_y
}
