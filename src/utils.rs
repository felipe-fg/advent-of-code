use num::Num;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::str::FromStr;

pub fn read_lines(filename: &str, ignore_empty: bool) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .filter_map(Result::ok)
        .filter(move |line| !ignore_empty || !line.is_empty())
}

pub fn read_numbers<'a, T: 'a>(filename: &str, split: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: Num + FromStr,
    <T as FromStr>::Err: Debug,
{
    read_lines(filename, true).flat_map(move |line| {
        line.split(split)
            .filter(|number| !number.is_empty())
            .map(|number| number.parse().expect(&format!("invalid number {}", number)))
            .collect::<Vec<T>>()
    })
}
