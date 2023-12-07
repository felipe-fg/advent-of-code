use rayon::prelude::*;
use std::ops::RangeInclusive;

type Number = u64;
type Range = RangeInclusive<Number>;
type CategoryMaps = Vec<CategoryMap>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Number>,
    categories: Vec<CategoryMaps>,
}

#[derive(Debug)]
struct CategoryMap {
    destination_range_start: Number,
    source_range_start: Number,
    range_length: Number,
}

impl CategoryMap {
    fn map(&self, range: &Range) -> (Option<Range>, Vec<Range>) {
        let source_start = self.source_range_start;
        let source_end = self.source_range_start + self.range_length - 1u64;

        if !(range.end() < &source_start || range.start() > &source_end) {
            let intersection_start = *range.start().max(&source_start);
            let intersection_end = *range.end().min(&source_end);

            let mut remainder_ranges = Vec::new();

            if range.start() < &intersection_start {
                let remainder_left = *range.start()..=intersection_start - 1u64;

                remainder_ranges.push(remainder_left);
            }

            if range.end() > &intersection_end {
                let remainder_right = intersection_end + 1u64..=*range.end();

                remainder_ranges.push(remainder_right);
            }

            let mapped_start =
                intersection_start - self.source_range_start + self.destination_range_start;
            let mapped_end =
                intersection_end - self.source_range_start + self.destination_range_start;

            let mapped_range = mapped_start..=mapped_end;

            (Some(mapped_range), remainder_ranges)
        } else {
            let remainder_ranges = vec![range.clone()];

            (None, remainder_ranges)
        }
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day05.txt", true).collect();

    let almanac = parse_almanac(&lines);

    let lowest_single = almanac
        .seeds
        .par_iter()
        .map(|&seed| compute_lowest_mapped_number(seed..=seed, &almanac.categories))
        .min()
        .expect("lowest_single");

    let seed_ranges: Vec<_> = almanac
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..=chunk[0] + chunk[1] - 1u64)
        .collect();

    let lowest_ranges = seed_ranges
        .par_iter()
        .map(|seed_range| compute_lowest_mapped_number(seed_range.clone(), &almanac.categories))
        .min()
        .expect("lowest_ranges");

    println!("{:?}", lowest_single);
    println!("{:?}", lowest_ranges);
}

fn parse_almanac(lines: &[String]) -> Almanac {
    let mut iter = lines.iter();

    let seeds: Vec<Number> = iter
        .next()
        .expect("seeds")
        .split_ascii_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();

    let mut categories = Vec::new();
    let mut current_category_maps = Vec::new();

    for line in iter {
        if line.contains("map:") && !current_category_maps.is_empty() {
            categories.push(current_category_maps);
            current_category_maps = Vec::new();
        } else if !line.contains("map:") && !line.trim().is_empty() {
            let numbers: Vec<Number> = line
                .split_ascii_whitespace()
                .filter_map(|part| part.parse().ok())
                .collect();

            let category_map = CategoryMap {
                destination_range_start: numbers[0],
                source_range_start: numbers[1],
                range_length: numbers[2],
            };

            current_category_maps.push(category_map);
        }
    }

    categories.push(current_category_maps);

    Almanac { seeds, categories }
}

fn compute_lowest_mapped_number(initial_range: Range, categories: &[CategoryMaps]) -> Number {
    let mut ranges = vec![initial_range.clone()];

    for category in categories.iter() {
        let mut category_mapped_ranges = Vec::new();

        for category_map in category {
            let mut category_map_remainder_ranges = Vec::new();

            for range in ranges.iter() {
                let (mapped_range, remainder_ranges) = category_map.map(range);

                if let Some(mapped_range) = mapped_range {
                    category_mapped_ranges.push(mapped_range);
                }

                category_map_remainder_ranges.extend(remainder_ranges);
            }

            ranges = category_map_remainder_ranges;
        }

        ranges.extend(category_mapped_ranges);
    }

    *ranges
        .iter()
        .map(|range| range.start())
        .min()
        .expect("lowest")
}
