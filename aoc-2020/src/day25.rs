use aoc_utils;

type Value = u128;

pub fn run() {
    let keys: Vec<Value> = aoc_utils::read_numbers("inputs/day25.txt", ",").collect();

    let subject_number = 7;
    let public_key_card = keys[0];
    let public_key_door = keys[1];

    let loop_size_card = calculate_loop_size(subject_number, public_key_card);

    let encryption_key = transform(public_key_door, loop_size_card);

    println!("{}", encryption_key);
}

fn transform(subject_number: Value, loop_size: Value) -> Value {
    let mut current_key = 1;

    for _ in 0..loop_size {
        current_key *= subject_number;
        current_key %= 20201227;
    }

    current_key
}

fn calculate_loop_size(subject_number: Value, key: Value) -> Value {
    let mut loop_size = 0;
    let mut current_key = 1;

    loop {
        loop_size += 1;
        current_key *= subject_number;
        current_key %= 20201227;

        if current_key == key {
            return loop_size;
        }
    }
}
