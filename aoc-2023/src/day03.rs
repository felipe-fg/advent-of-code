use itertools::Itertools;

type Schematic = Vec<Vec<char>>;
type Number = u32;
type Symbol = char;
type Position = (usize, usize);

const EMPTY: char = '.';

#[derive(Debug)]
struct PartNumber {
    number: Number,
    symbol: Option<Symbol>,
    symbol_position: Option<Position>,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day03.txt", true).collect();

    let schematic = parse_schematic(&lines);
    let part_numbers = parse_part_numbers(&schematic);

    let sum = compute_sum_valid_parts(&part_numbers);
    let ratio = compute_ratio_gears(&part_numbers);

    println!("{:?}", sum);
    println!("{:?}", ratio);
}

fn compute_sum_valid_parts(part_numbers: &[PartNumber]) -> u32 {
    part_numbers
        .iter()
        .filter(|part_number| part_number.symbol.is_some())
        .map(|part_number| part_number.number)
        .sum()
}

fn compute_ratio_gears(part_numbers: &[PartNumber]) -> u32 {
    let mut part_number_gears: Vec<_> = part_numbers
        .iter()
        .filter(|part_number| part_number.symbol == Some('*'))
        .collect();

    part_number_gears.sort_by_key(|part_number| part_number.symbol_position);

    part_number_gears
        .iter()
        .group_by(|part_number| part_number.symbol_position)
        .into_iter()
        .map(|(_, part_number_group)| {
            let part_number_group: Vec<_> = part_number_group.collect();

            if part_number_group.len() == 2 {
                part_number_group
                    .iter()
                    .map(|part_number| part_number.number)
                    .reduce(|a, b| a * b)
                    .unwrap_or_default()
            } else {
                0
            }
        })
        .sum()
}

fn parse_schematic(lines: &[String]) -> Schematic {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn parse_part_numbers(schematic: &Schematic) -> Vec<PartNumber> {
    let rows = schematic.len();
    let columns = schematic.first().map(|row| row.len()).unwrap_or(0);

    let mut part_numbers = Vec::new();
    let mut current_symbol: Option<Symbol> = None;
    let mut current_symbol_position: Option<Position> = None;
    let mut current_number: String = String::new();

    for y in 0..rows {
        let mut previous_char = EMPTY;

        for x in 0..columns {
            let current_char = schematic[y][x];

            if current_char.is_ascii_digit() {
                if let Some((symbol, symbol_position)) = find_symbol_at(schematic, x, y) {
                    current_symbol = Some(symbol);
                    current_symbol_position = Some(symbol_position);
                }

                current_number.push(current_char);
            } else if previous_char.is_ascii_digit() {
                push_part_number(
                    &mut part_numbers,
                    &mut current_symbol,
                    &mut current_symbol_position,
                    &mut current_number,
                );
            }

            previous_char = current_char;
        }

        if !current_number.is_empty() {
            push_part_number(
                &mut part_numbers,
                &mut current_symbol,
                &mut current_symbol_position,
                &mut current_number,
            );
        }
    }

    part_numbers
}

fn push_part_number(
    part_numbers: &mut Vec<PartNumber>,
    current_symbol: &mut Option<Symbol>,
    current_symbol_position: &mut Option<Position>,
    current_number: &mut String,
) {
    let part_number = PartNumber {
        number: current_number.parse().expect("number"),
        symbol: current_symbol.to_owned(),
        symbol_position: current_symbol_position.to_owned(),
    };

    part_numbers.push(part_number);

    *current_symbol = None;
    *current_number = String::new();
    *current_symbol_position = None;
}

fn find_symbol_at(schematic: &Schematic, x: usize, y: usize) -> Option<(Symbol, Position)> {
    let rows = schematic.len();
    let columns = schematic.first().map(|row| row.len()).unwrap_or(0);

    let min_x = (x as isize - 1).max(0) as usize;
    let max_x = (x as isize + 1).min(columns as isize - 1) as usize;
    let min_y = (y as isize - 1).max(0) as usize;
    let max_y = (y as isize + 1).min(rows as isize - 1) as usize;

    let mut symbols = Vec::new();

    for adjacent_y in min_y..=max_y {
        for adjacent_x in min_x..=max_x {
            let char = schematic[adjacent_y][adjacent_x];

            if !char.is_ascii_digit() && char != EMPTY {
                symbols.push((char, (adjacent_x, adjacent_y)));
            }
        }
    }

    symbols.into_iter().next()
}
