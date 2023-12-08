use regex::Regex;
use std::collections::HashMap;

type Instruction = char;
type Node = String;
type Number = u128;
type NodeStepMap = HashMap<Node, Number>;

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    network: HashMap<Node, (Node, Node)>,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day08.txt", true).collect();

    let map = parse_documents(&lines);

    let steps_by_node = count_steps_map(&map);

    let steps_aaa = steps_by_node.get("AAA").expect("node");

    let steps_match = find_match_goal(&steps_by_node);

    println!("{:?}", steps_aaa);
    println!("{:?}", steps_match);
}

fn parse_documents(lines: &[String]) -> Map {
    let re = Regex::new(r"(?P<node>\w+) = [(](?P<left>\w+), (?P<right>\w+)[)]").expect("regex");

    let mut iter = lines.iter();

    let instructions = iter.next().expect("instructions").chars().collect();

    let network = iter
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let caps = re.captures(line).expect("captures");

            let node = caps["node"].to_string();
            let left = caps["left"].to_string();
            let right = caps["right"].to_string();

            (node, (left, right))
        })
        .collect();

    Map {
        instructions,
        network,
    }
}

fn count_steps_map(map: &Map) -> NodeStepMap {
    let start_nodes: Vec<_> = map
        .network
        .iter()
        .filter(|(node, _)| node.ends_with('A'))
        .map(|(node, _)| node.as_str())
        .collect();

    start_nodes
        .iter()
        .map(|start_node| (start_node.to_string(), count_steps_node(map, start_node)))
        .collect()
}

fn count_steps_node(map: &Map, start_node: &str) -> Number {
    let mut steps = 0;
    let mut current_node = start_node;
    let mut instructions = map.instructions.iter().cycle();

    while !current_node.ends_with('Z') {
        let (left_node, right_node) = map.network.get(current_node).expect("next");

        let instruction = instructions.next().expect("instruction");

        match instruction {
            'L' => {
                current_node = left_node;
            }
            'R' => {
                current_node = right_node;
            }
            _ => unreachable!(),
        }

        steps += 1;
    }

    steps
}

fn find_match_goal(steps_by_node: &NodeStepMap) -> u128 {
    let mut step = 1;
    let mut increment = 1;

    for node_steps in steps_by_node.values() {
        loop {
            if step % node_steps == 0 {
                increment = num::integer::lcm(increment, *node_steps);
                break;
            } else {
                step += increment;
            }
        }
    }

    step
}
