use cached::proc_macro::cached;
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    other_numbers: Vec<u32>,
}

impl Card {
    fn matches(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|number| self.other_numbers.contains(number))
            .count() as u32
    }

    fn points(&self) -> u32 {
        let matches = self.matches();

        if matches == 0 {
            0
        } else {
            2u32.pow(matches - 1)
        }
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day04.txt", true).collect();

    let cards = parse_cards(&lines);

    let points: u32 = cards.iter().map(|card| card.points()).sum();
    let count = count_winning_cards(&cards);

    println!("{:?}", points);
    println!("{:?}", count);
}

fn parse_cards(lines: &[String]) -> Vec<Card> {
    let re = r"Card\s*(?P<id>\d+):(?P<winning>(?:\s*?\d+\s*?)+)[|](?P<other>(?:\s*?\d+\s*?)+)";
    let re = Regex::new(re).expect("regex");

    lines
        .iter()
        .map(|line| {
            let caps = re.captures(line).expect("captures");

            let id = caps["id"].to_string().parse().expect("id");

            let winning_numbers = caps["winning"]
                .split_ascii_whitespace()
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .map(|part| part.parse().expect("number"))
                .collect();

            let other_numbers = caps["other"]
                .split_ascii_whitespace()
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .map(|part| part.parse().expect("number"))
                .collect();

            Card {
                id,
                winning_numbers,
                other_numbers,
            }
        })
        .collect()
}

fn count_winning_cards(cards: &Vec<Card>) -> u32 {
    #[cached(key = "u32", convert = r#"{ card.id }"#)]
    fn count_loop(cards: &Vec<Card>, card: &Card) -> u32 {
        let matches = card.matches();

        let from = card.id as usize;
        let to = card.id as usize + matches as usize - 1usize;
        let next_cards = &cards[from..=to];

        1u32 + next_cards
            .par_iter()
            .map(|card| count_loop(cards, card))
            .sum::<u32>()
    }

    cards.par_iter().map(|card| count_loop(cards, card)).sum()
}
