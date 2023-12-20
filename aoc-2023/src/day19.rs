use std::ops::RangeInclusive;

use regex::Regex;

type Name = String;
type Rate = u128;
type Workflows = Vec<Workflow>;
type MachineParts = Vec<MachinePart>;

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";

#[derive(Debug, Clone)]
struct Workflow {
    name: Name,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
enum Rule {
    Condition(ConditionRule),
    Destination(DestinationRule),
}

#[derive(Debug, Clone)]
struct ConditionRule {
    variable: Variable,
    condition: Condition,
    value: Rate,
    destination: Name,
}

#[derive(Debug, Copy, Clone)]
enum Variable {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
struct DestinationRule {
    destination: String,
}

#[derive(Debug, Clone)]
struct MachinePart {
    x: RangeInclusive<Rate>,
    m: RangeInclusive<Rate>,
    a: RangeInclusive<Rate>,
    s: RangeInclusive<Rate>,
    destination: Option<Name>,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day19.txt", false).collect();

    let (workflows, machine_parts) = parse_workflows_parts(&lines);

    let workflow_in = workflows
        .iter()
        .find(|workflow| workflow.name == "in")
        .expect("in");

    let accepted: MachineParts = machine_parts
        .iter()
        .map(|machine_part| sort_machine_part(machine_part, workflow_in, &workflows))
        .flat_map(|(accepted, _)| accepted)
        .collect();

    let sum = sum_machine_parts(&accepted);

    let machine_part_combination = MachinePart {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
        destination: None,
    };

    let (accepted_combination, _) =
        sort_machine_part(&machine_part_combination, workflow_in, &workflows);

    let combination = combination_machine_parts(&accepted_combination);

    println!("{}", sum);
    println!("{}", combination);
}

fn parse_workflows_parts(lines: &[String]) -> (Workflows, MachineParts) {
    let mut workflows = Workflows::new();
    let mut machine_parts = MachineParts::new();

    let mut is_workflow_line = true;

    for line in lines {
        if line.is_empty() {
            is_workflow_line = false;
            continue;
        }

        if is_workflow_line {
            let workflow = parse_workflow(line);

            workflows.push(workflow);
        } else {
            let machine_part = parse_machine_part(line);

            machine_parts.push(machine_part);
        }
    }

    (workflows, machine_parts)
}

fn parse_workflow(line: &str) -> Workflow {
    let re = Regex::new(r"(?P<name>\w+)[{](?P<rules>.+)[}]").expect("regex");
    let caps = re.captures(line).expect("captures");

    let name = caps["name"].to_string();
    let rules = caps["rules"].split(',').map(parse_rule).collect();

    Workflow { name, rules }
}

fn parse_rule(line: &str) -> Rule {
    let re = r"(?:(?P<variable>[xmas])(?P<condition>[<>])(?P<value>\d+)[:])?(?P<destination>\w+)";
    let re = Regex::new(re).expect("regex");
    let caps = re.captures(line).expect("captures");

    let is_condition_rule = caps.name("variable").is_some();

    if is_condition_rule {
        let variable = match &caps["variable"] {
            "x" => Variable::X,
            "m" => Variable::M,
            "a" => Variable::A,
            "s" => Variable::S,
            _ => unreachable!(),
        };

        let condition = match &caps["condition"] {
            "<" => Condition::LessThan,
            ">" => Condition::GreaterThan,
            _ => unreachable!(),
        };

        let value = caps["value"].parse().expect("value");
        let destination = caps["destination"].to_string();

        let rule = ConditionRule {
            variable,
            condition,
            value,
            destination,
        };

        Rule::Condition(rule)
    } else {
        let destination = caps["destination"].to_string();

        let rule = DestinationRule { destination };

        Rule::Destination(rule)
    }
}

fn parse_machine_part(line: &str) -> MachinePart {
    let re = r"[{]x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)[}]";
    let re = Regex::new(re).expect("regex");
    let caps = re.captures(line).expect("captures");

    let x: Rate = caps["x"].parse().expect("x");
    let m: Rate = caps["m"].parse().expect("m");
    let a: Rate = caps["a"].parse().expect("a");
    let s: Rate = caps["s"].parse().expect("s");

    MachinePart {
        x: x..=x,
        m: m..=m,
        a: a..=a,
        s: s..=s,
        destination: None,
    }
}

fn sort_machine_part(
    machine_part: &MachinePart,
    workflow: &Workflow,
    workflows: &Workflows,
) -> (MachineParts, MachineParts) {
    let mut accepted = MachineParts::new();
    let mut rejected = MachineParts::new();

    let mut current_machine_part = machine_part.clone();

    for rule in &workflow.rules {
        let (mapped, remaining) = map_rule(&current_machine_part, rule);

        if let Some(mapped) = mapped {
            match &mapped.destination {
                Some(destination) if destination == ACCEPTED => {
                    accepted.push(mapped);
                }
                Some(destination) if destination == REJECTED => {
                    rejected.push(mapped);
                }
                Some(destination) => {
                    let next_workflow = workflows
                        .iter()
                        .find(|workflow| &workflow.name == destination)
                        .expect("workflow");

                    let (next_accepted, next_rejected) =
                        sort_machine_part(&mapped, next_workflow, workflows);

                    accepted.extend(next_accepted);
                    rejected.extend(next_rejected);
                }
                None => unreachable!(),
            }
        }

        if let Some(remaining) = remaining {
            current_machine_part = remaining;
        } else {
            break;
        }
    }

    (accepted, rejected)
}

fn map_rule(machine_part: &MachinePart, rule: &Rule) -> (Option<MachinePart>, Option<MachinePart>) {
    match rule {
        Rule::Condition(condition_rule) => {
            let variable_range = get_variable_range(machine_part, condition_rule);

            let (mapped_range, remaining_range) = map_range(
                variable_range,
                &condition_rule.condition,
                condition_rule.value,
            );

            let mapped_machine_part = mapped_range.map(|range| {
                let destination = Some(condition_rule.destination.clone());

                let (x, m, a, s) =
                    set_variable_range(machine_part, &condition_rule.variable, range);

                MachinePart {
                    x,
                    m,
                    a,
                    s,
                    destination,
                }
            });

            let remaining_machine_part = remaining_range.map(|range| {
                let (x, m, a, s) =
                    set_variable_range(machine_part, &condition_rule.variable, range);

                MachinePart {
                    x,
                    m,
                    a,
                    s,
                    ..machine_part.clone()
                }
            });

            (mapped_machine_part, remaining_machine_part)
        }
        Rule::Destination(destination_rule) => {
            let destination = Some(destination_rule.destination.clone());

            let mapped_machine_part = MachinePart {
                destination,
                ..machine_part.clone()
            };

            (Some(mapped_machine_part), None)
        }
    }
}

fn map_range(
    range: RangeInclusive<Rate>,
    condition: &Condition,
    value: Rate,
) -> (Option<RangeInclusive<Rate>>, Option<RangeInclusive<Rate>>) {
    match condition {
        Condition::LessThan => {
            let mapped = if *range.start() < value {
                Some(*range.start()..=(value - 1).min(*range.end()))
            } else {
                None
            };

            let remaining = if *range.end() >= value {
                Some(value.max(*range.start())..=*range.end())
            } else {
                None
            };

            (mapped, remaining)
        }
        Condition::GreaterThan => {
            let mapped = if *range.end() > value {
                Some((value + 1).max(*range.start())..=*range.end())
            } else {
                None
            };

            let remaining = if *range.start() <= value {
                Some(*range.start()..=value.min(*range.end()))
            } else {
                None
            };

            (mapped, remaining)
        }
    }
}

fn get_variable_range(
    machine_part: &MachinePart,
    condition_rule: &ConditionRule,
) -> RangeInclusive<Rate> {
    match condition_rule.variable {
        Variable::X => machine_part.x.clone(),
        Variable::M => machine_part.m.clone(),
        Variable::A => machine_part.a.clone(),
        Variable::S => machine_part.s.clone(),
    }
}

fn set_variable_range(
    machine_part: &MachinePart,
    variable: &Variable,
    range: RangeInclusive<Rate>,
) -> (
    RangeInclusive<Rate>,
    RangeInclusive<Rate>,
    RangeInclusive<Rate>,
    RangeInclusive<Rate>,
) {
    match variable {
        Variable::X => (
            range,
            machine_part.m.clone(),
            machine_part.a.clone(),
            machine_part.s.clone(),
        ),
        Variable::M => (
            machine_part.x.clone(),
            range,
            machine_part.a.clone(),
            machine_part.s.clone(),
        ),
        Variable::A => (
            machine_part.x.clone(),
            machine_part.m.clone(),
            range,
            machine_part.s.clone(),
        ),
        Variable::S => (
            machine_part.x.clone(),
            machine_part.m.clone(),
            machine_part.a.clone(),
            range,
        ),
    }
}

fn sum_machine_parts(machine_parts: &MachineParts) -> Rate {
    machine_parts
        .iter()
        .map(|a| a.x.start() + a.m.start() + a.a.start() + a.s.start())
        .sum()
}

fn combination_machine_parts(machine_parts: &MachineParts) -> Rate {
    machine_parts
        .iter()
        .map(|a| {
            (a.x.end() - a.x.start() + 1)
                * (a.m.end() - a.m.start() + 1)
                * (a.a.end() - a.a.start() + 1)
                * (a.s.end() - a.s.start() + 1)
        })
        .sum()
}
