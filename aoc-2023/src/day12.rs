use cached::proc_macro::cached;

type Number = u64;
type Records = Vec<Record>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    springs: Vec<State>,
    groups: Vec<Number>,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day12.txt", true).collect();

    let records = parse_records(&lines, false);
    let records_unfolded = parse_records(&lines, true);

    let sum: Number = records.iter().map(count_arrangements).sum();
    let sum_unfolded: Number = records_unfolded.iter().map(count_arrangements).sum();

    println!("{:?}", sum);
    println!("{:?}", sum_unfolded);
}

fn parse_records(lines: &[String], unfold: bool) -> Records {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();

            let springs: Vec<_> = parts
                .next()
                .expect("springs")
                .chars()
                .map(|part| match part {
                    '.' => State::Operational,
                    '#' => State::Damaged,
                    '?' => State::Unknown,
                    _ => unreachable!(),
                })
                .collect();

            let groups: Vec<Number> = parts
                .next()
                .expect("groups")
                .split(',')
                .filter_map(|part| part.parse().ok())
                .collect();

            if unfold {
                let springs = [
                    &springs[..],
                    &[State::Unknown],
                    &springs[..],
                    &[State::Unknown],
                    &springs[..],
                    &[State::Unknown],
                    &springs[..],
                    &[State::Unknown],
                    &springs[..],
                ]
                .concat();

                let groups = [
                    &groups[..],
                    &groups[..],
                    &groups[..],
                    &groups[..],
                    &groups[..],
                ]
                .concat();

                Record { springs, groups }
            } else {
                Record { springs, groups }
            }
        })
        .collect()
}

fn count_arrangements(record: &Record) -> Number {
    count_loop(record.springs.clone(), record.groups.clone())
}

#[cached()]
fn count_loop(springs: Vec<State>, groups: Vec<Number>) -> Number {
    let mut next_springs = springs.to_vec();
    let mut next_groups = groups.to_vec();

    let mut modified_group = None;

    for spring in springs {
        let spring = match (spring, modified_group) {
            // previous group has been modified to zero
            (State::Unknown, Some(0)) => State::Operational,
            // previous group has been modified
            (State::Unknown, Some(_)) => State::Damaged,
            _ => spring,
        };

        match (spring, modified_group, next_groups.first()) {
            // invalid state - unfinished group
            (State::Operational, Some(count), _) if count > 0 => return 0,
            // invalid state - no group remaining
            (State::Damaged, _, None) => return 0,
            // invalid state - previous group has been modified to zero
            (State::Damaged, Some(0), _) => return 0,
            // continue to next spring
            (State::Operational, _, _) => {
                modified_group = None;

                next_springs.remove(0);
            }
            // decrease group and continue to next spring
            (State::Damaged, _, Some(_)) => {
                if let Some(count) = next_groups.first_mut() {
                    *count -= 1;

                    modified_group = Some(*count);
                }

                if let Some(0) = next_groups.first() {
                    next_groups.remove(0);
                }

                next_springs.remove(0);
            }
            // fork spring into damaged and operational
            (State::Unknown, _, _) => break,
        }
    }

    match (&next_springs[..], &next_groups[..]) {
        // valid final state
        ([], []) => 1,
        // invalid final state - remaining group
        ([], [..]) => 0,
        // fork spring into damaged and operational
        _ => {
            let mut next_springs_damaged = next_springs.clone();
            next_springs_damaged.remove(0);
            next_springs_damaged.insert(0, State::Damaged);

            let mut next_springs_operational = next_springs.clone();
            next_springs_operational.remove(0);
            next_springs_operational.insert(0, State::Operational);

            let count_damaged = count_loop(next_springs_damaged, next_groups.clone());
            let count_operational = count_loop(next_springs_operational, next_groups.clone());

            count_damaged + count_operational
        }
    }
}
