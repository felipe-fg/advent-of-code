use super::utils;
use rayon::prelude::*;

type Expression = Vec<Token>;
type Value = i128;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Number(Value),
    Symbol(Operator),
    Bracket(Expression),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operator {
    Add,
    Mul,
}

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day18.txt", true).collect();

    let expressions_simple = parse_expressions(&lines);
    let expressions_advanced = parse_expressions(&lines);

    let sum_simple = evaluate_sum(expressions_simple, false);
    let sum_advanced = evaluate_sum(expressions_advanced, true);

    println!("{}", sum_simple);
    println!("{}", sum_advanced);
}

fn parse_expressions(lines: &Vec<String>) -> Vec<Expression> {
    lines
        .par_iter()
        .map(|line| parse_expression(line))
        .collect()
}

fn parse_expression(string: &str) -> Expression {
    let mut expression = vec![];

    let mut iter = string.chars().enumerate();

    while let Some((index, character)) = iter.next() {
        match character {
            ' ' => (),
            '+' => expression.push(Token::Symbol(Operator::Add)),
            '*' => expression.push(Token::Symbol(Operator::Mul)),
            '(' => {
                let close_index = find_close_bracket(string, index);

                let bracket_string = &string[index + 1..close_index];
                let bracket_token = Token::Bracket(parse_expression(bracket_string));

                expression.push(bracket_token);

                iter.nth(bracket_string.len());
            }
            _ => {
                let value = character.to_string().parse().expect("number");

                expression.push(Token::Number(value));
            }
        }
    }

    expression
}

fn find_close_bracket(string: &str, open_index: usize) -> usize {
    let characters = string
        .chars()
        .enumerate()
        .filter(|(index, _)| index > &open_index)
        .filter(|(_, character)| character == &'(' || character == &')');

    let mut control = 1;

    for (index, character) in characters {
        match character {
            '(' => control += 1,
            ')' => control -= 1,
            _ => (),
        }

        if control == 0 {
            return index;
        }
    }

    open_index
}

fn evaluate_sum(expressions: Vec<Expression>, advanced: bool) -> Value {
    expressions
        .into_par_iter()
        .map(|expression| evaluate(&expression, advanced))
        .sum()
}

fn evaluate(expression: &[Token], advanced: bool) -> Value {
    let mut expression: Expression = expression
        .iter()
        .map(|token| match token {
            Token::Bracket(expression) => Token::Number(evaluate(expression, advanced)),
            Token::Symbol(operator) => Token::Symbol(*operator),
            Token::Number(value) => Token::Number(*value),
        })
        .collect();

    if advanced {
        loop {
            let symbol_index = expression
                .iter()
                .position(|token| token == &Token::Symbol(Operator::Add));

            if let Some(symbol_index) = symbol_index {
                let left = &expression[symbol_index - 1];
                let right = &expression[symbol_index + 1];

                let value = match (left, right) {
                    (Token::Number(left), Token::Number(right)) => Token::Number(left + right),
                    _ => Token::Number(0),
                };

                expression.remove(symbol_index - 1);
                expression.remove(symbol_index - 1);
                expression.remove(symbol_index - 1);
                expression.insert(symbol_index - 1, value);
            } else {
                break;
            }
        }
    }

    let mut current_value = 0;
    let mut current_operator = Operator::Add;

    for token in expression {
        match token {
            Token::Bracket(_) => (),
            Token::Symbol(operator) => current_operator = operator,
            Token::Number(value) => match current_operator {
                Operator::Add => current_value += value,
                Operator::Mul => current_value *= value,
            },
        }
    }

    current_value
}
