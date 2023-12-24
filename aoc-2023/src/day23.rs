use std::collections::HashMap;

use colored::*;

type Number = i64;
type Position = (Number, Number);
type Positions = Vec<Position>;

type Distance = usize;
type Graph = HashMap<Position, HashMap<Position, Distance>>;

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    SlopeLeft,
    SlopeRight,
    SlopeUp,
    SlopeDown,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day23.txt", true).collect();

    let map = parse_map(&lines);

    let graph_slopes = build_map_graph(&map, (1, 0), true);
    let graph = build_map_graph(&map, (1, 0), false);

    let longest_slopes = find_longest_hike(&map, &graph_slopes);
    let longest = find_longest_hike(&map, &graph);

    draw_map(&map);
    println!("{}", longest_slopes);
    println!("{}", longest);
}

fn parse_map(lines: &[String]) -> Map {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '<' => Tile::SlopeLeft,
                    '>' => Tile::SlopeRight,
                    '^' => Tile::SlopeUp,
                    'v' => Tile::SlopeDown,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn build_map_graph(map: &Map, start: Position, slopes: bool) -> Graph {
    let start_node = (start, start, start, 0);

    let mut stack = vec![start_node];
    let mut visited = Positions::new();
    let mut graph = Graph::new();

    while let Some((origin, previous, position, distance)) = stack.pop() {
        visited.push(position);

        let neighbors = get_map_neighbors(map, position, previous, slopes);

        match &neighbors[..] {
            &[neighbor] => {
                let next = (origin, position, neighbor, distance + 1);

                stack.push(next);
            }
            [] => {
                graph.entry(origin).or_default().insert(position, distance);

                if !slopes {
                    graph.entry(position).or_default().insert(origin, distance);
                }
            }
            neighbors => {
                graph.entry(origin).or_default().insert(position, distance);

                if !slopes {
                    graph.entry(position).or_default().insert(origin, distance);
                }

                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        let next = (position, position, neighbor, 1);

                        stack.push(next);
                    }
                }
            }
        }
    }

    graph
}

fn get_map_neighbors(map: &Map, (x, y): Position, previous: Position, slopes: bool) -> Positions {
    let tile = get_map_tile(map, (x, y));

    let left = (-1, 0);
    let right = (1, 0);
    let up = (0, -1);
    let down = (0, 1);
    let all = vec![up, down, left, right];

    let directions = match tile {
        Some(Tile::SlopeLeft) if slopes => vec![left],
        Some(Tile::SlopeRight) if slopes => vec![right],
        Some(Tile::SlopeUp) if slopes => vec![up],
        Some(Tile::SlopeDown) if slopes => vec![down],
        Some(Tile::SlopeLeft | Tile::SlopeRight | Tile::SlopeUp | Tile::SlopeDown) => all,
        Some(Tile::Path) => all,
        _ => vec![],
    };

    directions
        .into_iter()
        .map(|(direction_x, direction_y)| (x + direction_x, y + direction_y))
        .filter(|&next_position| next_position != previous)
        .filter_map(|next_position| {
            get_map_tile(map, next_position)
                .filter(|tile| tile != &Tile::Forest)
                .map(move |_| next_position)
        })
        .collect()
}

fn get_map_tile(map: &Map, (x, y): Position) -> Option<Tile> {
    if x < 0 || y < 0 {
        None
    } else {
        map.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .cloned()
    }
}

fn find_longest_hike(map: &Map, graph: &Graph) -> Distance {
    let rows = map.len() as Number;
    let columns = map.first().map(|row| row.len()).unwrap_or_default() as Number;

    let start = (1, 0);
    let goal = (columns - 2, rows - 1);
    let visited = Positions::new();
    let distance = 0;

    dfs_longest_path(graph, start, goal, &visited, distance).expect("path")
}

fn dfs_longest_path(
    graph: &Graph,
    current: Position,
    goal: Position,
    visited: &Positions,
    distance: Distance,
) -> Option<Distance> {
    let mut visited = visited.to_vec();

    visited.push(current);

    if current == goal {
        Some(distance)
    } else {
        graph
            .get(&current)
            .iter()
            .flat_map(|neighbors| neighbors.iter())
            .filter(|(neighbor_position, _)| !visited.contains(neighbor_position))
            .flat_map(|(&neighbor_position, neighbor_distance)| {
                dfs_longest_path(
                    graph,
                    neighbor_position,
                    goal,
                    &visited,
                    distance + neighbor_distance,
                )
            })
            .max()
    }
}

fn draw_map(map: &Map) {
    for (_, row) in map.iter().enumerate() {
        for (_, tile) in row.iter().enumerate() {
            let tile = match tile {
                Tile::Path => "█".bright_green(),
                Tile::Forest => "█".green(),
                Tile::SlopeLeft => "⮜".black().on_bright_green(),
                Tile::SlopeRight => "⮞".black().on_bright_green(),
                Tile::SlopeUp => "⮝".black().on_bright_green(),
                Tile::SlopeDown => "⮟".black().on_bright_green(),
            };

            print!("{}", tile);
        }

        println!();
    }
}
