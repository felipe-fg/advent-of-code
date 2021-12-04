use aoc_utils;

#[derive(Debug, Clone)]
struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

#[derive(Debug, Clone)]
struct Board {
    data: Vec<Vec<usize>>,
}

impl Board {
    fn is_winner(&self, numbers: &[usize]) -> bool {
        let winner_row = self
            .data
            .iter()
            .any(|row| row.iter().all(|number| numbers.contains(number)));

        let winner_column = (0..self.data[0].len()).any(|column| {
            self.data
                .iter()
                .map(|row| row[column])
                .all(|number| numbers.contains(&number))
        });

        winner_row || winner_column
    }

    fn get_score(&self, numbers: &[usize]) -> usize {
        let sum_unmarked: usize = self
            .data
            .iter()
            .flat_map(|row| row)
            .filter(|number| !numbers.contains(number))
            .sum();

        let last_number = numbers.last().expect("last");

        sum_unmarked * last_number
    }
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day04.txt", false).collect();

    let bingo = parse_bingo(lines);

    let (first_numbers, first_board) = find_first_winner(&bingo);
    let (last_numbers, last_board) = find_last_winner(&bingo);

    println!("{:?}", first_board.get_score(first_numbers));
    println!("{:?}", last_board.get_score(last_numbers));
}

fn parse_bingo(lines: Vec<String>) -> Bingo {
    let line_numbers = &lines[0];
    let line_boards = &lines[2..];

    let numbers = parse_line_numbers(line_numbers);
    let boards = parse_line_boards(line_boards);

    Bingo { numbers, boards }
}

fn parse_line_numbers(line: &str) -> Vec<usize> {
    line.split(&[' ', ','][..])
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .map(|item| item.parse().expect("number"))
        .collect()
}

fn parse_line_boards(lines: &[String]) -> Vec<Board> {
    let mut boards = vec![];
    let mut data = vec![];

    for line in lines {
        if !line.trim().is_empty() {
            data.push(parse_line_numbers(line));
        } else if !data.is_empty() {
            boards.push(Board { data });
            data = vec![];
        }
    }

    if !data.is_empty() {
        boards.push(Board { data });
    }

    boards
}

fn find_first_winner(bingo: &Bingo) -> (&[usize], &Board) {
    (0..bingo.numbers.len())
        .filter_map(|round| {
            let numbers = &bingo.numbers[0..round + 1];

            bingo
                .boards
                .iter()
                .find(|board| board.is_winner(numbers))
                .map(|board| (numbers, board))
        })
        .next()
        .expect("winner")
}

fn find_last_winner(bingo: &Bingo) -> (&[usize], &Board) {
    (0..bingo.numbers.len())
        .rev()
        .filter_map(|round| {
            let numbers = &bingo.numbers[0..round + 1];

            bingo
                .boards
                .iter()
                .find(|board| !board.is_winner(numbers))
                .map(|board| {
                    let next_numbers = &bingo.numbers[0..round + 2];

                    (next_numbers, board)
                })
        })
        .next()
        .expect("winner")
}
