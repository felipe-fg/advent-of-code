use itertools::Itertools;

type Number = u64;
type Races = Vec<Race>;

#[derive(Debug)]
struct Race {
    time: Number,
    distance: Number,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day06.txt", true).collect();

    let races = parse_races(&lines, false);
    let races_merged = parse_races(&lines, true);

    let count = count_ways(&races);
    let count_merged = count_ways(&races_merged);

    println!("{:?}", count);
    println!("{:?}", count_merged);
}

fn parse_races(lines: &[String], merge: bool) -> Races {
    let mut iter = lines.iter();

    let times: Vec<Number> = iter
        .next()
        .expect("times")
        .split_ascii_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();

    let distances: Vec<Number> = iter
        .next()
        .expect("distances")
        .split_ascii_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();

    if merge {
        let time = times
            .iter()
            .map(|time| time.to_string())
            .join("")
            .parse()
            .expect("time");

        let distance = distances
            .iter()
            .map(|distance| distance.to_string())
            .join("")
            .parse()
            .expect("distance");

        vec![Race { time, distance }]
    } else {
        times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| Race { time, distance })
            .collect()
    }
}

fn count_ways(races: &[Race]) -> Number {
    races
        .iter()
        .map(|race| {
            let (min, max) = find_hold_times(race);

            max - min + 1
        })
        .reduce(|a, b| a * b)
        .expect("count")
}

fn find_hold_times(race: &Race) -> (Number, Number) {
    fn find_edge_win(start: Number, end: Number, race: &Race, invert: bool) -> Number {
        let middle = (start + end) / 2;

        let previous_win = distance(middle - 1, race.time) > race.distance;
        let current_win = distance(middle, race.time) > race.distance;

        let previous_win = if invert { !previous_win } else { previous_win };
        let current_win = if invert { !current_win } else { current_win };

        if current_win && !previous_win {
            if invert {
                middle - 1
            } else {
                middle
            }
        } else if !current_win && !previous_win {
            find_edge_win(middle + 1, end, race, invert)
        } else {
            find_edge_win(start, middle - 1, race, invert)
        }
    }

    let start = 1;
    let end = race.time - 1;
    let middle = (start + end) / 2;

    let min = find_edge_win(start, middle, race, false);
    let max = find_edge_win(middle, end, race, true);

    (min, max)
}

fn distance(hold_time: Number, race_time: Number) -> Number {
    hold_time * (race_time - hold_time)
}
