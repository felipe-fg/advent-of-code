use aoc_utils;

type Count = u128;
type TimerCount = Vec<Count>;

const MAX_TIMER: usize = 8;

pub fn run() {
    let numbers: Vec<usize> = aoc_utils::read_numbers("inputs/day06.txt", ",").collect();

    let timer_count = parse_timer_count(numbers);

    let simulation_80 = simulate(&timer_count, 80);
    let simulation_256 = simulate(&timer_count, 256);

    println!("{}", simulation_80.iter().sum::<Count>());
    println!("{}", simulation_256.iter().sum::<Count>());
}

fn parse_timer_count(numbers: Vec<usize>) -> TimerCount {
    let mut timer_count = vec![0; MAX_TIMER + 1];

    for number in numbers {
        timer_count[number] += 1
    }

    timer_count
}

fn simulate(initial_timer_count: &TimerCount, days: usize) -> TimerCount {
    (0..days).fold(initial_timer_count.clone(), |timer_count, _| {
        next_day(&timer_count)
    })
}

fn next_day(current_timer_count: &TimerCount) -> TimerCount {
    let mut next_timer_count = vec![0; MAX_TIMER + 1];

    for (timer, &count) in current_timer_count.iter().enumerate() {
        if timer == 0 {
            next_timer_count[6] += count;
            next_timer_count[8] += count;
        } else {
            next_timer_count[timer - 1] += count;
        }
    }

    next_timer_count
}
