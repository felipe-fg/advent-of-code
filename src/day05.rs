use super::utils;

pub fn run() {
    let mut seats: Vec<usize> = utils::read_lines("inputs/day05.txt", true)
        .map(|seat| find_row_column(&seat))
        .map(|(row, column)| row * 8 + column)
        .collect();

    seats.sort_unstable();

    let last = seats.last().expect("last");

    let empty = seats
        .windows(2)
        .find(|pair| match pair {
            [current, next] => current + 1 != *next,
            _ => false,
        })
        .expect("empty");

    println!("{}", last);
    println!("{:?}", empty);
}

fn find_row_column(seat: &str) -> (usize, usize) {
    let row = find_seat(&seat[..7], 0, 127);
    let column = find_seat(&seat[7..], 0, 7);

    (row, column)
}

fn find_seat(seat: &str, min: usize, max: usize) -> usize {
    let current = &seat[..1];
    let remaining = &seat[1..];

    let middle = min + ((max - min) / 2);

    let (min, max) = match current {
        "F" | "L" => (min, middle),
        "B" | "R" => (middle + 1, max),
        _ => (min, max),
    };

    if remaining.is_empty() {
        match current {
            "F" | "L" => min,
            "B" | "R" => max,
            _ => 128,
        }
    } else {
        find_seat(remaining, min, max)
    }
}
