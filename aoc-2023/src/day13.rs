type Pattern = Vec<Vec<Tile>>;
type Patterns = Vec<Pattern>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Ash,
    Rock,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day13.txt", false).collect();

    let patterns = parse_patterns(&lines);

    let summary = summarize_patterns(&patterns, false);
    let summary_smudge = summarize_patterns(&patterns, true);

    println!("{}", summary);
    println!("{}", summary_smudge);
}

fn parse_patterns(lines: &[String]) -> Patterns {
    let mut patterns = Patterns::new();

    let mut pattern = Pattern::new();

    for line in lines {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = Pattern::new();
        } else {
            let row = line
                .chars()
                .map(|char| match char {
                    '.' => Tile::Ash,
                    '#' => Tile::Rock,
                    _ => unreachable!(),
                })
                .collect();

            pattern.push(row);
        }
    }

    patterns.push(pattern);

    patterns
}

fn summarize_patterns(patterns: &Patterns, smudge_mode: bool) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            let (vertical, horizontal) = count_reflections(pattern, smudge_mode);

            vertical + 100 * horizontal
        })
        .sum()
}

fn count_reflections(pattern: &Pattern, smudge_mode: bool) -> (usize, usize) {
    let horizontal = count_horizontal_reflections(pattern, smudge_mode).unwrap_or_default();

    let transposed = transpose_pattern(pattern);

    let vertical = count_horizontal_reflections(&transposed, smudge_mode).unwrap_or_default();

    (vertical, horizontal)
}

fn count_horizontal_reflections(pattern: &Pattern, smudge_mode: bool) -> Option<usize> {
    pattern
        .windows(2)
        .enumerate()
        .find(|&(index, _)| is_horizontal_reflection(pattern, index, smudge_mode))
        .map(|(index, _)| index + 1)
}

fn is_horizontal_reflection(pattern: &Pattern, top_index: usize, smudge_mode: bool) -> bool {
    let remaining_top = top_index + 1;
    let remaining_bottom = pattern.len() - remaining_top;
    let remaining = remaining_top.min(remaining_bottom);

    let mut smudge_fixed = false;

    for index in 0..remaining {
        let top_row = &pattern[top_index - index];
        let bottom_row = &pattern[top_index + 1 + index];

        let differences = count_differences(top_row, bottom_row);

        if smudge_mode && differences == 1 && !smudge_fixed {
            smudge_fixed = true;
        } else if differences > 0 {
            return false;
        }
    }

    if smudge_mode {
        smudge_fixed
    } else {
        true
    }
}

fn count_differences(a: &[Tile], b: &[Tile]) -> usize {
    a.iter().zip(b.iter()).filter(|(a, b)| a != b).count()
}

fn transpose_pattern(pattern: &Pattern) -> Pattern {
    let rows = pattern.len();
    let columns = pattern.first().map(|row| row.len()).unwrap_or_default();

    (0..columns)
        .map(|column| (0..rows).map(|row| pattern[row][column]).collect())
        .collect()
}
