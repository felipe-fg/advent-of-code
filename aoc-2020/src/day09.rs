use aoc_utils;

pub fn run() {
    let numbers: Vec<u128> = aoc_utils::read_lines("inputs/day09.txt", true)
        .map(|line| line.parse().expect("number"))
        .collect();

    let invalid = find_invalid_number(&numbers, 25);
    let range = find_contiguous_range(&numbers, invalid);
    let sum = range.iter().min().expect("min") + range.iter().max().expect("max");

    println!("{}", invalid);
    println!("{:?}", sum);
}

fn find_invalid_number(numbers: &[u128], preamble: usize) -> u128 {
    for (index, number) in numbers.iter().enumerate().skip(preamble) {
        let preamble_numbers = &numbers[index - preamble..index];

        if !is_number_valid(preamble_numbers, *number) {
            return *number;
        }
    }

    0
}

fn is_number_valid(preamble_numbers: &[u128], number: u128) -> bool {
    for a in preamble_numbers {
        for b in preamble_numbers {
            if a + b == number {
                return true;
            }
        }
    }

    false
}

fn find_contiguous_range(numbers: &[u128], invalid: u128) -> &[u128] {
    for (from, from_number) in numbers.iter().enumerate() {
        let mut sum = *from_number;

        for (to, to_number) in numbers.iter().enumerate().skip(from + 1) {
            sum += to_number;

            if sum == invalid {
                return &numbers[from..to + 1];
            } else if sum > invalid {
                break;
            }
        }
    }

    &[]
}
