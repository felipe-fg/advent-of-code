use aoc_utils;

type Precision = u128;

type Turn = Precision;
type Roll = Precision;
type Count = Precision;

type Position = Precision;
type Score = Precision;

const GAME_POSITIONS: Position = 10;
const DICE_ROLLS: Position = 3;
const SCORE_DETERMINISTIC: Score = 1000;
const SCORE_QUANTUM: Score = 21;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day21.txt", true).collect();

    let (player1, player2) = parse_positions(lines);

    let (count, score) = play_deterministic(player1, player2);
    let (count1, count2) = play_quantum(player1, player2);

    println!("{:?}", count * score);
    println!("{:?}", count1.max(count2));
}

fn parse_positions(lines: Vec<String>) -> (Position, Position) {
    fn read_position(line: Option<&String>) -> Position {
        line.expect("line")
            .chars()
            .last()
            .expect("last")
            .to_string()
            .parse()
            .expect("number")
    }

    let player1 = read_position(lines.first());
    let player2 = read_position(lines.last());

    (player1, player2)
}

fn play_deterministic(start_current: Position, start_next: Position) -> (Count, Score) {
    fn game_loop(
        position_current: Position,
        position_next: Position,
        score_current: Score,
        score_next: Score,
        turn: Turn,
    ) -> (Count, Score) {
        let (roll, count) = roll_deterministic(turn);

        let position_current = wrap_position(position_current + roll);
        let score_current = score_current + position_current;
        let turn = turn + 1;

        if score_current >= SCORE_DETERMINISTIC {
            (count, score_next)
        } else {
            game_loop(
                position_next,
                position_current,
                score_next,
                score_current,
                turn,
            )
        }
    }

    game_loop(start_current, start_next, 0, 0, 0)
}

fn roll_deterministic(turn: Turn) -> (Roll, Count) {
    let first = turn * DICE_ROLLS + 1;
    let last = (turn + 1) * DICE_ROLLS;

    let roll = (first + last) * DICE_ROLLS / 2;

    (roll, last)
}

fn play_quantum(start_current: Position, start_next: Position) -> (Count, Count) {
    fn game_loop(
        position_current: Position,
        position_next: Position,
        score_current: Score,
        score_next: Score,
        current_roll: Roll,
        first_player: bool,
    ) -> (Count, Count) {
        let position_current = wrap_position(position_current + current_roll);
        let score_current = score_current + position_current;

        if score_current >= SCORE_QUANTUM {
            if first_player {
                (1, 0)
            } else {
                (0, 1)
            }
        } else {
            fork_game(
                position_next,
                position_current,
                score_next,
                score_current,
                !first_player,
            )
        }
    }

    fn fork_game(
        position_current: Position,
        position_next: Position,
        score_current: Score,
        score_next: Score,
        first_player: bool,
    ) -> (Count, Count) {
        let rolls = roll_quantum();

        rolls
            .into_iter()
            .map(|(current_roll, roll_count)| {
                let (count1, count2) = game_loop(
                    position_current,
                    position_next,
                    score_current,
                    score_next,
                    current_roll,
                    first_player,
                );

                (count1 * roll_count, count2 * roll_count)
            })
            .reduce(|(left1, right1), (left2, right2)| (left1 + left2, right1 + right2))
            .expect("fork")
    }

    fork_game(start_current, start_next, 0, 0, true)
}

fn roll_quantum() -> Vec<(Roll, Count)> {
    vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
}

fn wrap_position(position: Position) -> Position {
    (position - 1) % GAME_POSITIONS + 1
}
