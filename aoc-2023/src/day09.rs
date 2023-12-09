type Number = i64;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day09.txt", true).collect();

    let histories = parse_histories(&lines);

    let sum_forward: Number = histories
        .iter()
        .map(|history| sum_extrapolated_values(history))
        .sum();

    let sum_backward: Number = histories
        .iter()
        .map(|history| history.iter().cloned().rev().collect::<Vec<_>>())
        .map(|history| sum_extrapolated_values(&history))
        .sum();

    println!("{:?}", sum_forward);
    println!("{:?}", sum_backward);
}

fn parse_histories(lines: &[String]) -> Vec<Vec<Number>> {
    lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|part| part.parse().ok())
                .collect()
        })
        .collect()
}

fn sum_extrapolated_values(history: &[Number]) -> Number {
    let sequences = generate_difference_sequences(history);

    let mut sum = 0;

    for sequence in sequences.iter().rev() {
        let last = sequence.last().expect("last");

        sum += last
    }

    sum
}

fn generate_difference_sequences(sequence: &[Number]) -> Vec<Vec<Number>> {
    let mut sequences = vec![sequence.to_vec()];
    let mut sequence = sequences.last().expect("last");

    while !is_sequence_zero(sequence) {
        let next_sequence = generate_difference_sequence(sequence);

        sequences.push(next_sequence);
        sequence = sequences.last().expect("last");
    }

    sequences
}

fn generate_difference_sequence(sequence: &[Number]) -> Vec<Number> {
    sequence
        .windows(2)
        .map(|numbers| numbers[1] - numbers[0])
        .collect()
}

fn is_sequence_zero(sequence: &[Number]) -> bool {
    sequence.iter().all(|number| number == &0)
}
