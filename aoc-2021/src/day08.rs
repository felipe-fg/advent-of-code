use aoc_utils;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

type SevenSegmentDisplay = String;
type Pattern = SevenSegmentDisplay;
type Output = SevenSegmentDisplay;
type FourDigitDisplay = (Vec<Pattern>, Vec<Output>);
type DecodedFourDigitDisplay = Vec<usize>;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day08.txt", true).collect();

    let parsed_displays = parse_four_digit_displays(lines);

    let decoded_displays = decode_four_digit_displays(&parsed_displays);

    let count = count_digits(&decoded_displays);
    let sum = sum_displays(&decoded_displays);

    println!("{}", count);
    println!("{}", sum);
}

fn parse_four_digit_displays(lines: Vec<String>) -> Vec<FourDigitDisplay> {
    lines
        .par_iter()
        .map(|line| parse_four_digit_display(line))
        .collect()
}

fn parse_four_digit_display(line: &str) -> FourDigitDisplay {
    let mut parts = line.split("|");

    let patterns = parse_seven_segment_displays(parts.next().expect("patterns"));
    let outputs = parse_seven_segment_displays(parts.next().expect("outputs"));

    (patterns, outputs)
}

fn parse_seven_segment_displays(line: &str) -> Vec<SevenSegmentDisplay> {
    line.split_ascii_whitespace()
        .map(|display| display.trim())
        .filter(|display| !display.is_empty())
        .map(|display| display.chars().sorted().join(""))
        .collect()
}

fn count_digits(displays: &[DecodedFourDigitDisplay]) -> usize {
    displays
        .par_iter()
        .flat_map(|display| display)
        .filter(|&&digit| digit == 1 || digit == 4 || digit == 7 || digit == 8)
        .count()
}

fn sum_displays(displays: &[DecodedFourDigitDisplay]) -> usize {
    displays
        .par_iter()
        .map(|display| display.iter().join("").parse::<usize>().expect("display"))
        .sum()
}

fn decode_four_digit_displays(displays: &[FourDigitDisplay]) -> Vec<DecodedFourDigitDisplay> {
    displays
        .par_iter()
        .map(|display| decode_four_digit_display(display))
        .collect()
}

fn decode_four_digit_display((patterns, outputs): &FourDigitDisplay) -> DecodedFourDigitDisplay {
    let decoded_patterns = decode_patterns(patterns);

    outputs
        .par_iter()
        .map(|output| decoded_patterns.get(output).expect("output").to_owned())
        .collect()
}

fn decode_patterns(patterns: &[Pattern]) -> HashMap<String, usize> {
    let digit_mask_segment = vec![
        (1, -1, 2),
        (4, -1, 4),
        (7, -1, 3),
        (8, -1, 7),
        (3, 1, 3),
        (6, 1, 5),
        (0, 3, 2),
        (5, 6, 0),
        (2, 4, 3),
        (9, 4, 2),
    ];

    let mut decoded_patterns: HashMap<isize, String> = HashMap::new();
    let mut remaining_patterns: Vec<&Pattern> = patterns.iter().collect();

    for (digit, mask, segment) in digit_mask_segment {
        let pattern_mask = decoded_patterns
            .get(&mask)
            .map(|pattern| pattern.to_string())
            .unwrap_or(String::from(""));

        let pattern_index = remaining_patterns
            .iter()
            .position(|pattern| count_segments(pattern, &pattern_mask) == segment)
            .expect("position");

        let pattern = remaining_patterns.remove(pattern_index).to_string();

        decoded_patterns.insert(digit, pattern);
    }

    decoded_patterns
        .into_iter()
        .map(|(digit, pattern)| (pattern, digit as usize))
        .collect()
}

fn count_segments(pattern: &Pattern, mask: &Pattern) -> usize {
    pattern
        .chars()
        .filter(|segments| !mask.chars().contains(segments))
        .count()
}
