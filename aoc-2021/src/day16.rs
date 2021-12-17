use aoc_utils;
use itertools::Itertools;

type Number = u64;
type PacketVersion = Number;
type PacketType = Number;
type LiteralValue = Number;
type OperatorPackets = Vec<Packet>;

#[derive(Debug, Clone)]
enum Packet {
    Literal(PacketVersion, PacketType, LiteralValue),
    Operator(PacketVersion, PacketType, OperatorPackets),
}

pub fn run() {
    let lines: Vec<String> = aoc_utils::read_lines("inputs/day16.txt", true).collect();

    let transmission = parse_transmission(lines);

    let packet = parse_packet(&mut transmission.into_iter());

    let (version, value) = evaluate_packet(&packet);

    println!("{}", version);
    println!("{}", value);
}

fn parse_transmission(lines: Vec<String>) -> Vec<String> {
    let hexadecimal = lines.iter().next().expect("transmission");

    let binary = to_binary(hexadecimal);

    binary.chars().map(|bit| bit.to_string()).collect()
}

fn parse_packet(it: &mut dyn Iterator<Item = String>) -> Packet {
    let packet_version = take_decimal(it, 3);
    let packet_type = take_decimal(it, 3);

    match packet_type {
        4 => {
            let literal_value = parse_literal(it);

            Packet::Literal(packet_version, packet_type, literal_value)
        }
        0..=3 | 5..=7 => {
            let operator_packets = parse_operator(it);

            Packet::Operator(packet_version, packet_type, operator_packets)
        }
        _ => panic!(),
    }
}

fn parse_literal(it: &mut dyn Iterator<Item = String>) -> LiteralValue {
    let mut literal_bits = vec![];

    loop {
        let group_prefix = take_decimal(it, 1);
        let group_bits = take_string(it, 4);

        literal_bits.push(group_bits);

        if group_prefix == 0 {
            break;
        }
    }

    let literal_bits = literal_bits.join("");

    to_decimal(&literal_bits)
}

fn parse_operator(it: &mut dyn Iterator<Item = String>) -> OperatorPackets {
    let mut operator_packets = vec![];

    let operator_length_type = take_decimal(it, 1);

    match operator_length_type {
        0 => {
            let bits_packets = take_decimal(it, 15);

            let mut it = it.take(bits_packets as usize).peekable();

            while it.peek().is_some() {
                let operator_packet = parse_packet(&mut it);

                operator_packets.push(operator_packet);
            }
        }
        1 => {
            let count_packets = take_decimal(it, 11);

            for _ in 0..count_packets {
                let operator_packet = parse_packet(it);

                operator_packets.push(operator_packet);
            }
        }
        _ => panic!(),
    }

    operator_packets
}

fn evaluate_packet(packet: &Packet) -> (PacketVersion, LiteralValue) {
    match packet {
        &Packet::Literal(packet_version, _, literal_value) => (packet_version, literal_value),
        Packet::Operator(packet_version, packet_type, operator_packets) => {
            let evaluations: Vec<(PacketVersion, LiteralValue)> = operator_packets
                .iter()
                .map(|packet| evaluate_packet(packet))
                .collect();

            let versions = evaluations.iter().map(|&(version, _)| version);
            let values = evaluations.iter().map(|&(_, value)| value);

            let version = packet_version + versions.sum::<PacketVersion>();

            let value = match packet_type {
                0 => values.sum(),
                1 => values.product(),
                2 => values.min().expect("min"),
                3 => values.max().expect("max"),
                5 => values.reduce(|a, b| if a > b { 1 } else { 0 }).expect(">"),
                6 => values.reduce(|a, b| if a < b { 1 } else { 0 }).expect("<"),
                7 => values.reduce(|a, b| if a == b { 1 } else { 0 }).expect("="),
                _ => panic!(),
            };

            (version, value)
        }
    }
}

fn take_decimal(it: &mut dyn Iterator<Item = String>, count: usize) -> Number {
    to_decimal(&take_string(it, count))
}

fn take_string(it: &mut dyn Iterator<Item = String>, count: usize) -> String {
    it.take(count).join("")
}

fn to_binary(hexadecimal: &str) -> String {
    hexadecimal
        .to_uppercase()
        .chars()
        .map(|character| match character {
            '0' => "0000".to_string(),
            '1' => "0001".to_string(),
            '2' => "0010".to_string(),
            '3' => "0011".to_string(),
            '4' => "0100".to_string(),
            '5' => "0101".to_string(),
            '6' => "0110".to_string(),
            '7' => "0111".to_string(),
            '8' => "1000".to_string(),
            '9' => "1001".to_string(),
            'A' => "1010".to_string(),
            'B' => "1011".to_string(),
            'C' => "1100".to_string(),
            'D' => "1101".to_string(),
            'E' => "1110".to_string(),
            'F' => "1111".to_string(),
            _ => panic!(),
        })
        .join("")
}

fn to_decimal(binary: &str) -> Number {
    Number::from_str_radix(binary, 2).expect("decimal")
}
