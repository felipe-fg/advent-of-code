use aoc_utils;
use std::collections::{HashMap, HashSet};

type Node = String;
type Edge = (Node, Node);
type NodeSet = HashSet<Node>;
type Graph = HashMap<Node, NodeSet>;

const NODE_START: &str = "start";
const NODE_END: &str = "end";

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day12.txt", true).collect();

    let graph = parse_graph(lines);

    let paths_once = compute_paths(&graph, false);
    let paths_twice = compute_paths(&graph, true);

    println!("{}", paths_once.len());
    println!("{}", paths_twice.len());
}

fn parse_graph(lines: Vec<String>) -> Graph {
    let edges: Vec<Edge> = lines
        .iter()
        .flat_map(|line| parse_edge_data(line))
        .filter(|(left, right)| right != NODE_START && left != NODE_END)
        .collect();

    parse_graph_data(&edges)
}

fn parse_edge_data(line: &str) -> Vec<Edge> {
    let mut parts = line.split("-");

    let left = parts.next().expect("left").trim();
    let right = parts.next().expect("right").trim();

    vec![
        (left.to_string(), right.to_string()),
        (right.to_string(), left.to_string()),
    ]
}

fn parse_graph_data(edges: &[Edge]) -> Graph {
    let mut graph = Graph::new();

    for (left, right) in edges {
        graph
            .entry(left.to_string())
            .and_modify(|neighbors| {
                neighbors.insert(right.to_string());
            })
            .or_insert_with(|| {
                let mut neighbors = NodeSet::new();
                neighbors.insert(right.to_string());
                neighbors
            });
    }

    graph
}

fn compute_paths(graph: &Graph, twice: bool) -> Vec<String> {
    fn compute_loop(graph: &Graph, node: Node, previous: NodeSet, twice: bool) -> Vec<String> {
        if node == NODE_END {
            return vec![node.to_string()];
        }

        let twice = twice && !previous.contains(&node);

        let neighbors = graph
            .get(&node)
            .map(|neighbors| {
                if twice {
                    neighbors.clone()
                } else {
                    &neighbors.clone() - &previous
                }
            })
            .unwrap_or(NodeSet::new());

        neighbors
            .into_iter()
            .flat_map(|neighbor| {
                let mut previous = previous.clone();

                if node.to_lowercase() == node {
                    previous.insert(node.to_string());
                }

                let paths = compute_loop(graph, neighbor, previous, twice);

                paths
                    .into_iter()
                    .map(|path| format!("{},{}", node, path))
                    .collect::<Vec<String>>()
            })
            .collect()
    }

    compute_loop(graph, NODE_START.to_string(), NodeSet::new(), twice)
}
