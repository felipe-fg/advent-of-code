use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use regex::Regex;

type ModuleName = String;
type ModuleDestination = Vec<ModuleName>;
type Configuration = HashMap<ModuleName, (ModuleType, ModuleDestination)>;

type Pulse = (PulseValue, ModuleName, ModuleName);
type Pulses = VecDeque<Pulse>;
type ButtonPress = u64;
type PulseCount = u64;

type MemoryAddress = String;
type MemoryBlock = HashMap<MemoryAddress, PulseValue>;
type MemoryBlocks = HashMap<ModuleName, MemoryBlock>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PulseValue {
    Low,
    High,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("aoc-2023/inputs/day20.txt", true).collect();

    let configuration = parse_configuration(&lines);

    let (count_low_pulses, count_high_pulses, _) = run_configuration(&configuration, 1000);
    let (_, _, conjunction_pulses) = run_configuration(&configuration, 10000);

    let button_presses = find_button_presses_low_pulse(&configuration, &conjunction_pulses);

    println!("{}", count_low_pulses * count_high_pulses);
    println!("{}", button_presses);
}

fn parse_configuration(lines: &[String]) -> Configuration {
    lines
        .iter()
        .map(|line| {
            let re = r"(?P<type>[%&])?(?P<name>\w+) -> (?P<destination>.+)";
            let re = Regex::new(re).expect("regex");

            let caps = re.captures(line).expect("captures");

            let module_name = caps["name"].to_string();
            let module_type = caps.name("type").map(|group| group.as_str());

            let module_type = match module_type {
                Some("%") => ModuleType::FlipFlop,
                Some("&") => ModuleType::Conjunction,
                _ if module_name == "broadcaster" => ModuleType::Broadcast,
                _ => unreachable!(),
            };

            let destination = caps["destination"]
                .split(',')
                .map(|name| name.trim().to_string())
                .collect();

            (module_name, (module_type, destination))
        })
        .collect()
}

fn run_configuration(
    configuration: &Configuration,
    button_presses: ButtonPress,
) -> (PulseCount, PulseCount, Vec<(ButtonPress, Pulse)>) {
    let mut memory_blocks = initialize_memory(configuration);

    let mut count_low_pulses: PulseCount = 0;
    let mut count_high_pulses: PulseCount = 0;
    let mut conjunction_pulses: Vec<(ButtonPress, Pulse)> = Vec::new();

    let conjunction_before_rx = get_module_origin(configuration, &String::from("rx"))
        .first()
        .cloned()
        .expect("conjunction");

    for button_press in 1..=button_presses {
        let button_pulse = (
            PulseValue::Low,
            String::from("button"),
            String::from("broadcaster"),
        );

        let mut pulses = Pulses::new();
        pulses.push_back(button_pulse);

        while let Some((pulse_value, pulse_origin, pulse_destination)) = pulses.pop_front() {
            match pulse_value {
                PulseValue::Low => {
                    count_low_pulses += 1;
                }
                PulseValue::High => {
                    count_high_pulses += 1;

                    if pulse_destination == conjunction_before_rx {
                        conjunction_pulses.push((
                            button_press,
                            (pulse_value, pulse_origin.clone(), pulse_destination.clone()),
                        ));
                    }
                }
            }

            if let Some((module_type, module_destination)) = configuration.get(&pulse_destination) {
                let memory_block = memory_blocks
                    .get_mut(&pulse_destination)
                    .expect("memory block");

                match module_type {
                    ModuleType::FlipFlop => {
                        if pulse_value == PulseValue::Low {
                            let state = memory_block.get_mut(&pulse_destination).expect("state");

                            *state = match state {
                                PulseValue::Low => PulseValue::High,
                                PulseValue::High => PulseValue::Low,
                            };

                            let destination_value = *state;

                            for destination_name in module_destination {
                                let pulse = (
                                    destination_value,
                                    pulse_destination.clone(),
                                    destination_name.clone(),
                                );

                                pulses.push_back(pulse);
                            }
                        }
                    }
                    ModuleType::Conjunction => {
                        let state = memory_block.get_mut(&pulse_origin).expect("state");

                        *state = pulse_value;

                        let inputs_high = memory_block
                            .iter()
                            .all(|(_, state)| state == &PulseValue::High);

                        let destination_value = if inputs_high {
                            PulseValue::Low
                        } else {
                            PulseValue::High
                        };

                        for destination_name in module_destination {
                            let pulse = (
                                destination_value,
                                pulse_destination.clone(),
                                destination_name.clone(),
                            );

                            pulses.push_back(pulse);
                        }
                    }
                    ModuleType::Broadcast => {
                        let destination_value = pulse_value;

                        for destination_name in module_destination {
                            let pulse = (
                                destination_value,
                                pulse_destination.clone(),
                                destination_name.clone(),
                            );

                            pulses.push_back(pulse);
                        }
                    }
                }
            }
        }
    }

    (count_low_pulses, count_high_pulses, conjunction_pulses)
}

fn initialize_memory(configuration: &Configuration) -> MemoryBlocks {
    let mut memory_blocks = MemoryBlocks::new();

    for (module_name, (module_type, _)) in configuration {
        let mut memory_block = MemoryBlock::new();

        match module_type {
            ModuleType::FlipFlop => {
                memory_block.insert(module_name.clone(), PulseValue::Low);
            }
            ModuleType::Conjunction => {
                let module_origin = get_module_origin(configuration, module_name);

                for module_origin_name in module_origin {
                    memory_block.insert(module_origin_name.clone(), PulseValue::Low);
                }
            }
            ModuleType::Broadcast => {}
        }

        memory_blocks.insert(module_name.clone(), memory_block);
    }

    memory_blocks
}

fn get_module_origin(configuration: &Configuration, module_name: &ModuleName) -> Vec<ModuleName> {
    configuration
        .iter()
        .filter(|(_, (_, destination))| destination.contains(module_name))
        .map(|(module_origin_name, (_, _))| module_origin_name)
        .unique()
        .cloned()
        .collect()
}

fn find_button_presses_low_pulse(
    configuration: &Configuration,
    conjunction_pulses: &[(ButtonPress, Pulse)],
) -> ButtonPress {
    let conjunction_before_rx = get_module_origin(configuration, &String::from("rx"))
        .first()
        .cloned()
        .expect("conjunction");

    let button_presses: Vec<_> = get_module_origin(configuration, &conjunction_before_rx)
        .iter()
        .map(|conjunction_name| {
            conjunction_pulses
                .iter()
                .filter(|(_, (_, origin_name, _))| conjunction_name == origin_name)
                .map(|(button_presses, (_, _, _))| button_presses)
                .min()
                .copied()
                .expect("min")
        })
        .collect();

    button_presses
        .into_iter()
        .reduce(num::integer::lcm)
        .expect("lcm")
}
