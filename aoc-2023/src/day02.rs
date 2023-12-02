#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day02.txt", true).collect();

    let games = parse_games(&lines);

    let sum_games: u32 = find_games(&games, 12, 13, 14).iter().map(|g| g.id).sum();
    let sum_powers: u32 = find_powers(&games).iter().sum();

    println!("{:?}", sum_games);
    println!("{:?}", sum_powers);
}

fn parse_games(lines: &[String]) -> Vec<Game> {
    lines.iter().map(|line| parse_game(line)).collect()
}

fn parse_game(line: &str) -> Game {
    let mut parts = line.split(':');

    let id = parts
        .next()
        .expect("id")
        .replace("Game ", "")
        .parse()
        .expect("id");

    let sets = parts
        .next()
        .expect("sets")
        .split(';')
        .map(parse_set)
        .collect();

    Game { id, sets }
}

fn parse_set(line: &str) -> Set {
    let mut set = Set::default();

    for part in line.split(',') {
        let mut parts = part.trim().split(' ');

        let count = parts.next().expect("count").parse().expect("number");
        let color = parts.next().expect("color");

        match color {
            "red" => set.red = count,
            "green" => set.green = count,
            "blue" => set.blue = count,
            _ => {}
        }
    }

    set
}

fn find_games(games: &[Game], red: u32, green: u32, blue: u32) -> Vec<&Game> {
    games
        .iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| set.red <= red && set.green <= green && set.blue <= blue)
        })
        .collect()
}

fn find_powers(games: &[Game]) -> Vec<u32> {
    games
        .iter()
        .map(|game| {
            let red = game.sets.iter().map(|set| set.red).max().expect("red");
            let green = game.sets.iter().map(|set| set.green).max().expect("green");
            let blue = game.sets.iter().map(|set| set.blue).max().expect("blue");

            red * green * blue
        })
        .collect()
}
