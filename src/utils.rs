use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

pub fn read_lines(filename: &str, ignore_empty: bool) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .filter_map(Result::ok)
        .filter(move |line| !ignore_empty || !line.is_empty())
}
