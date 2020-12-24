use super::utils;

type Deck = Vec<Card>;
type DeckSlice<'a> = &'a [Card];
type Card = usize;
type Score = u64;

#[derive(Eq, PartialEq)]
enum Player {
    One,
    Two,
}

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day22.txt", true).collect();

    let (one, two) = parse_decks(&lines);

    let (winning_combat, _) = play_game(&one, &two, false);
    let (winning_recursive_combat, _) = play_game(&one, &two, true);

    let score_combat = calculate_score(&winning_combat);
    let score_recursive_combat = calculate_score(&winning_recursive_combat);

    println!("{:?}", score_combat);
    println!("{:?}", score_recursive_combat);
}

fn parse_decks<T>(lines: &[T]) -> (Deck, Deck)
where
    T: AsRef<str>,
{
    let mut one = vec![];
    let mut two = vec![];

    let mut control = "";

    for line in lines {
        let line = line.as_ref();

        match &line[..] {
            "Player 1:" => control = "one",
            "Player 2:" => control = "two",
            _ => {
                let value = line.parse().expect("number");

                match control {
                    "one" => one.push(value),
                    "two" => two.push(value),
                    _ => (),
                }
            }
        }
    }

    (one, two)
}

fn play_game(start_one: DeckSlice, start_two: DeckSlice, recursive: bool) -> (Deck, Player) {
    let mut history: Vec<(Deck, Deck)> = vec![];

    let (mut one, mut two) = next_game_state(&start_one, &start_two, &history, recursive);

    while !one.is_empty() && !two.is_empty() {
        let (next_one, next_two) = next_game_state(&one, &two, &history, recursive);

        history.push((one.clone(), two.clone()));

        one = next_one;
        two = next_two;
    }

    if one.is_empty() {
        (two, Player::Two)
    } else {
        (one, Player::One)
    }
}

fn next_game_state(
    one: DeckSlice,
    two: DeckSlice,
    history: &[(Deck, Deck)],
    recursive: bool,
) -> (Deck, Deck) {
    let top_one = one[0];
    let top_two = two[0];

    if recursive {
        let previous_state = history
            .iter()
            .any(|(history_one, history_two)| one == &history_one[..] && two == &history_two[..]);

        let recursive_state = one.len() - 1 >= top_one && two.len() - 1 >= top_two;

        if previous_state {
            let next_one = one.iter().map(|value| *value).collect();
            let next_two = vec![];

            return (next_one, next_two);
        } else if recursive_state {
            let recursive_one = &one[1..1 + top_one];
            let recursive_two = &two[1..1 + top_two];

            let (_, winner) = play_game(recursive_one, recursive_two, true);

            if winner == Player::One {
                return next_deck_state(one, two, true);
            } else {
                return next_deck_state(two, one, false);
            }
        }
    }

    if top_one > top_two {
        next_deck_state(one, two, true)
    } else {
        next_deck_state(two, one, false)
    }
}

fn next_deck_state(winner: DeckSlice, loser: DeckSlice, keep_order: bool) -> (Deck, Deck) {
    let top_winner = winner[0];
    let top_loser = loser[0];

    let next_winner = winner[1..]
        .iter()
        .map(|card| *card)
        .chain(vec![top_winner, top_loser])
        .collect();

    let next_loser = loser[1..].iter().map(|card| *card).collect();

    if keep_order {
        (next_winner, next_loser)
    } else {
        (next_loser, next_winner)
    }
}

fn calculate_score(deck: DeckSlice) -> Score {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(index, card)| *card as Score * (index + 1) as Score)
        .sum()
}
