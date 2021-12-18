use aoc_utils;
use itertools::Itertools;

type Value = u64;

#[derive(Debug, Copy, Clone)]
enum Token {
    LeftBracket,
    Number(Value),
    RightBracket,
}

type TokenVec = Vec<Token>;

const THRESHOLD_EXPLODE: Value = 4;
const THRESHOLD_SPLIT: Value = 10;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day18.txt", true).collect();

    let list = parse_list(lines);

    let all = compute_assignment_all(&list);
    let permutations = compute_assignment_permutations(&list);

    println!("{}", all);
    println!("{}", permutations);
}

fn parse_list(lines: Vec<String>) -> Vec<TokenVec> {
    lines.iter().map(|line| parse_token_vec(line)).collect()
}

fn parse_token_vec(line: &str) -> TokenVec {
    let mut token_vec = vec![];

    for character in line.chars() {
        match character {
            '[' => token_vec.push(Token::LeftBracket),
            ']' => token_vec.push(Token::RightBracket),
            ',' => (),
            _ => {
                let number = character.to_string().parse().expect("number");

                token_vec.push(Token::Number(number));
            }
        }
    }

    token_vec
}

fn compute_assignment_all(list: &[TokenVec]) -> Value {
    let list: Vec<TokenVec> = list.iter().map(|item| item.clone()).collect();

    let total = list
        .into_iter()
        .reduce(|left, right| add_token_vec(&left, &right))
        .expect("total");

    compute_magnitude(&total)
}

fn compute_assignment_permutations(list: &[TokenVec]) -> Value {
    list.iter()
        .permutations(2)
        .map(|permutation| {
            let left = permutation[0];
            let right = permutation[1];

            let total = add_token_vec(left, right);

            compute_magnitude(&total)
        })
        .max()
        .expect("max")
}

fn add_token_vec(left: &TokenVec, right: &TokenVec) -> TokenVec {
    let mut token_vec = vec![];

    token_vec.push(Token::LeftBracket);
    token_vec.extend(left);
    token_vec.extend(right);
    token_vec.push(Token::RightBracket);

    reduce_token_vec(&token_vec)
}

fn reduce_token_vec(token_vec: &TokenVec) -> TokenVec {
    let mut token_vec = token_vec.clone();

    loop {
        if let Some(next) = try_explode(&token_vec) {
            token_vec = next;
            continue;
        } else if let Some(next) = try_split(&token_vec) {
            token_vec = next;
            continue;
        } else {
            break;
        }
    }

    token_vec
}

fn try_explode(token_vec: &TokenVec) -> Option<TokenVec> {
    try_explode_index(token_vec).map(|(index, left, right)| {
        let mut token_vec = token_vec.clone();

        token_vec.remove(index);
        token_vec.remove(index);
        token_vec.remove(index);
        token_vec.remove(index);

        token_vec.insert(index, Token::Number(0));

        for previous_index in (0..index).rev() {
            if let Token::Number(previous) = token_vec[previous_index] {
                token_vec[previous_index] = Token::Number(previous + left);
                break;
            }
        }

        for next_index in index + 1..token_vec.len() {
            if let Token::Number(next) = token_vec[next_index] {
                token_vec[next_index] = Token::Number(next + right);
                break;
            }
        }

        token_vec
    })
}

fn try_explode_index(token_vec: &TokenVec) -> Option<(usize, Value, Value)> {
    let mut depth = 0;

    for (index, token) in token_vec.iter().enumerate() {
        match token {
            Token::LeftBracket => depth += 1,
            Token::RightBracket => depth -= 1,
            &Token::Number(left) => {
                if depth >= THRESHOLD_EXPLODE + 1 {
                    if let Token::Number(right) = token_vec[index + 1] {
                        return Some((index - 1, left, right));
                    }
                }
            }
        }
    }

    None
}

fn try_split(token_vec: &TokenVec) -> Option<TokenVec> {
    try_split_index(token_vec).map(|(index, number)| {
        let mut token_vec = token_vec.clone();

        token_vec.remove(index + 1);

        token_vec.insert(index + 1, Token::LeftBracket);
        token_vec.insert(index + 2, Token::Number(number / 2));
        token_vec.insert(index + 3, Token::Number(number / 2 + number % 2));
        token_vec.insert(index + 4, Token::RightBracket);

        token_vec
    })
}

fn try_split_index(token_vec: &TokenVec) -> Option<(usize, Value)> {
    for (index, token) in token_vec.iter().enumerate() {
        if let &Token::Number(number) = token {
            if number >= THRESHOLD_SPLIT {
                return Some((index - 1, number));
            }
        }
    }

    None
}

fn compute_magnitude(token_vec: &TokenVec) -> Value {
    fn compute(it: &mut dyn Iterator<Item = &Token>) -> Value {
        let mut numbers = vec![];

        while let Some(&token) = it.next() {
            if let Token::LeftBracket = token {
                let number = compute(it);

                numbers.push(number);
            } else if let Token::Number(number) = token {
                numbers.push(number);
            } else if let Token::RightBracket = token {
                break;
            }
        }

        if numbers.len() == 2 {
            numbers[0] * 3 + numbers[1] * 2
        } else {
            numbers[0]
        }
    }

    compute(&mut token_vec.iter())
}
