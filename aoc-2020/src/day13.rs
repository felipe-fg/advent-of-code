use aoc_utils;

type ID = u128;
type Timestamp = u128;

#[derive(Debug)]
struct Notes {
    depart: u128,
    buses: Vec<(ID, Timestamp)>,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day13.txt", true).collect();

    let notes = parse_notes(&lines);

    let (bus, wait) = find_bus_wait(&notes);

    let depart = find_match_depart(&notes);

    println!("{}", bus * wait);
    println!("{:?}", depart);
}

fn parse_notes(lines: &Vec<String>) -> Notes {
    let depart = lines.get(0).expect("depart").parse().expect("number");

    let buses: Vec<(ID, Timestamp)> = lines
        .get(1)
        .expect("buses")
        .split(",")
        .enumerate()
        .filter(|(_, id)| id != &"x")
        .map(|(timestamp, id)| (id.parse().expect("number"), timestamp as u128))
        .collect();

    Notes {
        depart: depart,
        buses: buses,
    }
}

fn find_bus_wait(notes: &Notes) -> (u128, u128) {
    let (id, depart) = notes
        .buses
        .iter()
        .map(|(id, _)| {
            let depart = (notes.depart as f32 / *id as f32).ceil() as u128 * id;

            (*id, depart)
        })
        .min_by_key(|(_, depart)| *depart)
        .expect("bus");

    let wait = depart - notes.depart;

    (id, wait)
}

fn find_match_depart(notes: &Notes) -> u128 {
    let mut current_timestamp = 0;
    let mut current_step = 1;

    for (bus_id, bus_offset) in &notes.buses {
        loop {
            if (current_timestamp + bus_offset) % bus_id == 0 {
                current_step = lcm_prime(current_step, *bus_id);
                break;
            } else {
                current_timestamp += current_step;
            }
        }
    }

    current_timestamp
}

fn lcm_prime(a: u128, b: u128) -> u128 {
    a * b
}
