use fancy_regex::Regex;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day01.txt", true).collect();

    let sum_digits: u32 = parse_numbers(&lines, true).iter().sum();
    let sum_all: u32 = parse_numbers(&lines, false).iter().sum();

    println!("{}", sum_digits);
    println!("{}", sum_all);
}

fn parse_numbers(lines: &[String], only_digits: bool) -> Vec<u32> {
    let re = if only_digits {
        Regex::new(r"(?=(\d))")
    } else {
        Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine|ten))")
    }
    .expect("regex");

    lines
        .iter()
        .map(|line| {
            let mut digits = re
                .captures_iter(line)
                .map(|caps| caps.expect("captures").get(1).expect("group").as_str())
                .map(|group| match group {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    digit => digit,
                });

            let first = digits.next().expect("first");
            let last = digits.last().unwrap_or(first);

            format!("{first}{last}").parse().expect("number")
        })
        .collect()
}
