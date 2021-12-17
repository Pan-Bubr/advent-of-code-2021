use std::{iter::Sum, vec};

fn main() {
    let binary_input: String = parse_input(include_str!("input.txt"));

    let (packet, part_1) = parse_packet(&binary_input);

    dbg!(part_1);

    let part_2 = evaluate_packet(&packet);
    dbg!(part_2);
}

#[derive(Debug, Clone, Copy)]
enum ExitCondition {
    AtSubpacket(u64),
    AtEndLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    Value(u64),
    SubPacket(Op, Vec<Packet>),
}

fn get_operation(packet_number: u64) -> Op {
    match packet_number {
        0 => Op::Sum,
        1 => Op::Product,
        2 => Op::Minimum,
        3 => Op::Maximum,
        5 => Op::GreaterThan,
        6 => Op::LessThan,
        7 => Op::Equal,
        _ => panic!("Wrong number"),
    }
}

fn parse_subpacket(input_line: &str, exit_condition: ExitCondition) -> (Vec<Packet>, u64, String) {
    let mut ret_packet_version = 0;

    let mut remaining_string = input_line.to_string();
    let mut exit = false;
    let mut packets: Vec<Packet> = vec![];

    while !exit {
        let mut line = remaining_string.chars();

        let first_bit = line.next();

        if first_bit.is_none() {
            break;
        }

        let packet_version: String = [
            first_bit.unwrap(),
            line.next().unwrap(),
            line.next().unwrap(),
        ]
        .iter()
        .collect();

        let packet_version: u64 = u64::from_str_radix(&packet_version, 2).unwrap();

        ret_packet_version += packet_version;

        let packet_type_id: String = [
            line.next().unwrap(),
            line.next().unwrap(),
            line.next().unwrap(),
        ]
        .iter()
        .collect();
        let packet_type_id: u64 = u64::from_str_radix(&packet_type_id, 2).unwrap();

        match packet_type_id {
            4 => {
                let mut value_string = String::new();

                let mut exit_bit = false;

                while !exit_bit {
                    let first_bit = line.next().unwrap();
                    value_string.push(line.next().unwrap());
                    value_string.push(line.next().unwrap());
                    value_string.push(line.next().unwrap());
                    value_string.push(line.next().unwrap());

                    if first_bit == '0' {
                        exit_bit = true;
                    }
                }

                packets.push(Packet::Value(
                    u64::from_str_radix(&value_string, 2).unwrap(),
                ));

                remaining_string = line.collect();
            }
            packet_type => {
                let exit_condition_bit = line.next().unwrap();
                let exit_condition: ExitCondition;
                let mut subpacket_length = String::new();
                let mut subpackets: Vec<Packet> = vec![];

                if exit_condition_bit == '0' {
                    for _ in 0..15 {
                        subpacket_length.push(line.next().unwrap());
                    }
                    exit_condition = ExitCondition::AtEndLine;

                    let len = u64::from_str_radix(&subpacket_length, 2).unwrap();

                    let substring: String = line.collect();

                    let parsing = parse_subpacket(&substring[0..len as usize], exit_condition);
                    subpackets = parsing.0;
                    ret_packet_version += parsing.1;

                    remaining_string = substring[len as usize..].to_string();
                } else {
                    for _ in 0..11 {
                        subpacket_length.push(line.next().unwrap());
                    }
                    exit_condition = ExitCondition::AtSubpacket(
                        u64::from_str_radix(&subpacket_length, 2).unwrap(),
                    );

                    let substring: String = line.collect();

                    let parsing = parse_subpacket(&substring, exit_condition);
                    subpackets = parsing.0;
                    ret_packet_version += parsing.1;
                    remaining_string = parsing.2;
                }

                packets.push(Packet::SubPacket(get_operation(packet_type), subpackets));
            }
        }

        match exit_condition {
            ExitCondition::AtEndLine => {
                exit = remaining_string.len() == 0;
            }
            ExitCondition::AtSubpacket(n) => {
                exit = packets.len() == n as usize;
            }
        }
    }

    (packets, ret_packet_version, remaining_string)
}

fn parse_packet(input_line: &str) -> (Packet, u64) {
    let mut line = input_line.chars();
    let packet_version: String = [
        line.next().unwrap(),
        line.next().unwrap(),
        line.next().unwrap(),
    ]
    .iter()
    .collect();

    let packet_version: u64 = u64::from_str_radix(&packet_version, 2).unwrap();

    let packet_type_id: String = [
        line.next().unwrap(),
        line.next().unwrap(),
        line.next().unwrap(),
    ]
    .iter()
    .collect();
    let packet_type_id: u64 = u64::from_str_radix(&packet_type_id, 2).unwrap();

    match packet_type_id {
        4 => {
            let mut value_string = String::new();

            let mut exit_bit = false;

            while !exit_bit {
                let first_bit = line.next().unwrap();
                value_string.push(line.next().unwrap());
                value_string.push(line.next().unwrap());
                value_string.push(line.next().unwrap());
                value_string.push(line.next().unwrap());

                if first_bit == '0' {
                    exit_bit = true;
                }
            }

            (
                Packet::Value(u64::from_str_radix(&value_string, 2).unwrap()),
                packet_version,
            )
        }
        packet_type => {
            let exit_condition_bit = line.next().unwrap();
            let exit_condition: ExitCondition;
            let mut subpacket_length = String::new();
            let subpackets: Vec<Packet>;
            let subpacket_version: u64;

            if exit_condition_bit == '0' {
                for _ in 0..15 {
                    subpacket_length.push(line.next().unwrap());
                }
                exit_condition = ExitCondition::AtEndLine;

                let len = u64::from_str_radix(&subpacket_length, 2).unwrap();

                let substring: String = line.collect();

                let parsing = parse_subpacket(&substring[0..len as usize], exit_condition);
                subpackets = parsing.0;
                subpacket_version = parsing.1;
            } else {
                for _ in 0..11 {
                    subpacket_length.push(line.next().unwrap());
                }
                exit_condition =
                    ExitCondition::AtSubpacket(u64::from_str_radix(&subpacket_length, 2).unwrap());

                let substring: String = line.collect();

                let parsing = parse_subpacket(&substring, exit_condition);
                subpackets = parsing.0;
                subpacket_version = parsing.1;
            }

            (
                Packet::SubPacket(get_operation(packet_type), subpackets),
                packet_version + subpacket_version,
            )
        }
    }
}

fn parse_input(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        })
        .collect()
}

fn evaluate_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::Value(n) => *n,
        Packet::SubPacket(op, subpackets) => match op {
            Op::Sum => subpackets.iter().map(evaluate_packet).sum(),
            Op::Product => subpackets.iter().map(evaluate_packet).product(),
            Op::Minimum => subpackets.iter().map(evaluate_packet).min().unwrap(),
            Op::Maximum => subpackets.iter().map(evaluate_packet).max().unwrap(),
            Op::GreaterThan => {
                (evaluate_packet(&subpackets[0]) > evaluate_packet(&subpackets[1])) as u64
            }
            Op::LessThan => {
                (evaluate_packet(&subpackets[0]) < evaluate_packet(&subpackets[1])) as u64
            }
            Op::Equal => {
                (evaluate_packet(&subpackets[0]) == evaluate_packet(&subpackets[1])) as u64
            }
        },
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "D2FE28";
        assert_eq!(parse_input(input), "110100101111111000101000");
    }

    #[test]
    fn test_parse_packet() {
        let input = "D2FE28";
        let line = parse_input(input);
        assert_eq!(parse_packet(&line).0, Packet::Value(2021));
    }

    #[test]
    fn test_parse_packet_2() {
        let input = "EE00D40C823060";
        let line = parse_input(input);
        assert_eq!(
            parse_packet(&line).0,
            Packet::SubPacket(
                Op::Maximum,
                vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)]
            )
        );
    }
    #[test]
    fn test_parse_packet_3() {
        let input = "38006F45291200";
        let line = parse_input(input);
        assert_eq!(
            parse_packet(&line).0,
            Packet::SubPacket(Op::LessThan, vec![Packet::Value(10), Packet::Value(20)])
        );
    }

    #[test]
    fn test_parse_packet_4() {
        let input = "8A004A801A8002F478";
        let line = parse_input(input);
        assert_eq!(parse_packet(&line).1, 16);
    }

    #[test]
    fn test_parse_packet_5() {
        let input = "620080001611562C8802118E34";
        let line = parse_input(input);
        assert_eq!(
            parse_packet(&line).0,
            Packet::SubPacket(
                Op::Sum,
                vec![
                    Packet::SubPacket(Op::Sum, vec![Packet::Value(10), Packet::Value(11)]),
                    Packet::SubPacket(Op::Sum, vec![Packet::Value(12), Packet::Value(13)])
                ]
            )
        );
        assert_eq!(parse_packet(&line).1, 12);
    }

    #[test]
    fn test_parse_packet_6() {
        let input = "C0015000016115A2E0802F182340";
        let line = parse_input(input);
        assert_eq!(parse_packet(&line).1, 23);
    }

    #[test]
    fn test_parse_packet_7() {
        let input = "C200B40A82";
        let line = parse_input(input);
        assert_eq!(
            parse_packet(&line).0,
            Packet::SubPacket(Op::Sum, vec![Packet::Value(1), Packet::Value(2)])
        )
    }

    #[test]
    fn test_evaluate_packet_7() {
        let input = "C200B40A82";
        let line = parse_input(input);
        let packet = parse_packet(&line).0;
        assert_eq!(evaluate_packet(&packet), 3)
    }
    #[test]
    fn test_evaluate_packet_8() {
        let input = "04005AC33890";
        let line = parse_input(input);
        let packet = parse_packet(&line).0;
        assert_eq!(evaluate_packet(&packet), 54)
    }
    #[test]
    fn test_evaluate_packet_9() {
        let input = "880086C3E88112";
        let line = parse_input(input);
        let packet = parse_packet(&line).0;
        assert_eq!(evaluate_packet(&packet), 7)
    }
    #[test]
    fn test_evaluate_packet_10() {
        let input = "CE00C43D881120";
        let line = parse_input(input);
        let packet = parse_packet(&line).0;
        assert_eq!(evaluate_packet(&packet), 9)
    }
    #[test]
    fn test_evaluate_packet_11() {
        let input = "D8005AC2A8F0";
        let line = parse_input(input);
        let packet = parse_packet(&line).0;
        assert_eq!(evaluate_packet(&packet), 1)
    }

    #[test]
    fn test_evaluate_packet_12() {
        let input = "F600BC2D8F";
        let line = parse_input(input);
        let packet = parse_packet(&line).0;
        assert_eq!(evaluate_packet(&packet), 0)
    }
    #[test]
    fn test_evaluate_packet_13() {
        let input = "9C005AC2F8F0";
        let line = parse_input(input);
        let packet = parse_packet(&line).0;
        assert_eq!(evaluate_packet(&packet), 0)
    }
    #[test]
    fn test_evaluate_packet_14() {
        let input = "9C0141080250320F1802104A08";
        let line = parse_input(input);
        let packet = parse_packet(&line).0;
        assert_eq!(evaluate_packet(&packet), 1)
    }
}
