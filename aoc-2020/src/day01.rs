use aoc_utils;

const SUM: i32 = 2020;

pub fn run() {
    let numbers: Vec<i32> = aoc_utils::read_lines("inputs/day01.txt", true)
        .map(|line| line.parse().expect("number"))
        .collect();

    for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if a + b + c == SUM {
                    println!("A + B + C: {:?}", a * b * c);
                }
            }

            if a + b == SUM {
                println!("A + B: {:?}", a * b);
            }
        }
    }
}
