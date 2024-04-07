use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

type Component = String;
type Components = Vec<Component>;
type Connection = (String, String);
type Diagram = HashMap<Component, Components>;

const COUNT_WIRES: usize = 3;

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day25.txt", true).collect();

    let diagram = parse_diagram(&lines);

    let (size_a, size_b) = find_diagram_groups(diagram, COUNT_WIRES);

    println!("{}", size_a * size_b);
}

fn parse_diagram(lines: &[String]) -> Diagram {
    let mut diagram = Diagram::new();

    for line in lines {
        let mut parts = line.split(':');

        let component = parts.next().expect("component").trim().to_string();

        let connected_components: Vec<_> = parts
            .next()
            .expect("connected components")
            .trim()
            .split_ascii_whitespace()
            .map(|component| component.trim().to_string())
            .collect();

        diagram
            .entry(component.clone())
            .or_default()
            .extend(connected_components.clone());

        for connected_component in connected_components {
            diagram
                .entry(connected_component.clone())
                .or_default()
                .push(component.clone());
        }
    }

    diagram
}

fn find_diagram_groups(diagram: Diagram, expected_minimum_cut: usize) -> (usize, usize) {
    (usize::MIN..=usize::MAX)
        .into_par_iter()
        .map(|_| {
            let minimum_cut_diagram = find_minimum_cut(&diagram);

            let minimum_cut = minimum_cut_diagram
                .values()
                .map(|components| components.len())
                .next()
                .expect("minimum cut");

            let mut group_sizes = minimum_cut_diagram
                .keys()
                .map(|component| component.split('-').count());

            let group_a = group_sizes.next().expect("group a");
            let group_b = group_sizes.next().expect("group b");

            (minimum_cut, group_a, group_b)
        })
        .find_first(|&(minimum_cut, _, _)| minimum_cut == expected_minimum_cut)
        .map(|(_, group_a, group_b)| (group_a, group_b))
        .expect("diagram groups")
}

fn find_minimum_cut(diagram: &Diagram) -> Diagram {
    let mut diagram = diagram.clone();

    while diagram.len() > 2 {
        let connection = find_random_connection(&diagram);

        contract_connection(&mut diagram, connection);
    }

    diagram
}

fn find_random_connection(diagram: &Diagram) -> Connection {
    let (component, connected_components) =
        diagram.iter().choose(&mut thread_rng()).expect("component");

    let connected_component = connected_components
        .choose(&mut thread_rng())
        .expect("connected component");

    (component.clone(), connected_component.clone())
}

fn contract_connection(diagram: &mut Diagram, (component_a, component_b): Connection) {
    let component_c = format!("{}-{}", component_a, component_b);

    let connected_components_c: Vec<_> = diagram
        .get(&component_a)
        .zip(diagram.get(&component_b))
        .map(|(connected_components_a, connected_components_b)| {
            connected_components_a
                .iter()
                .chain(connected_components_b.iter())
                .filter(|&component| component != &component_a && component != &component_b)
                .cloned()
                .collect()
        })
        .expect("connected components c");

    diagram
        .entry(component_c.clone())
        .or_default()
        .extend(connected_components_c.clone());

    for connected_component_c in &connected_components_c {
        let other_components = diagram
            .get_mut(connected_component_c)
            .expect("other components");

        let count_connections_a_b = other_components
            .iter()
            .filter(|&component| component == &component_a || component == &component_b)
            .count();

        for _ in 0..count_connections_a_b {
            other_components.push(component_c.clone());
        }

        other_components.retain(|component| component != &component_a && component != &component_b);
    }

    diagram.remove(&component_a);
    diagram.remove(&component_b);
}
