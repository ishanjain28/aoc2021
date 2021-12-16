#![feature(test)]
use std::collections::HashMap;

extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input(input: &'static str) -> String {
    let mapping = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
    let input = input.trim();

    let mut out = String::with_capacity(input.len() * 4);

    for c in input.chars() {
        let v = *mapping.get(&c).unwrap();
        out.push_str(v);
    }

    out
}

fn read_header(s: &mut impl Iterator<Item = char>) -> (u8, u8) {
    let raw_version: String = s.by_ref().take(3).collect();
    let version = u8::from_str_radix(&raw_version, 2).unwrap();

    let raw_type_id: String = s.take(3).collect();
    let type_id = u8::from_str_radix(&raw_type_id, 2).unwrap();

    (version, type_id)
}

fn read_literal(input: &mut impl Iterator<Item = char>) -> (u64, u64) {
    let mut out = String::new();
    let mut read = 0;
    loop {
        let last = input.next().unwrap() == '0';
        out.extend(input.take(4));
        read += 5;

        if last {
            break;
        }
    }

    (u64::from_str_radix(&out, 2).unwrap(), read)
}

#[derive(Debug)]

struct PacketHeader {
    version: u8,
    type_id: u8,
}

#[derive(Debug)]
struct Packet {
    header: PacketHeader,
    size: u64,
    literal: Option<u64>,

    sub_packets: Vec<Packet>,
}

impl Packet {
    fn value(&self) -> u64 {
        match self.header.type_id {
            0 => self.sub_packets.iter().fold(0, |a, x| a + x.value()),
            1 => self.sub_packets.iter().fold(1, |a, x| a * x.value()),
            2 => self.sub_packets.iter().map(|x| x.value()).min().unwrap(),
            3 => self.sub_packets.iter().map(|x| x.value()).max().unwrap(),
            4 => self.literal.unwrap(),
            5 => {
                let value1 = self.sub_packets[0].value();
                let value2 = self.sub_packets[1].value();

                if value1 > value2 {
                    1
                } else {
                    0
                }
            }
            6 => {
                let value1 = self.sub_packets[0].value();
                let value2 = self.sub_packets[1].value();

                if value1 < value2 {
                    1
                } else {
                    0
                }
            }
            7 => {
                let value1 = self.sub_packets[0].value();
                let value2 = self.sub_packets[1].value();

                if value1 == value2 {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }
}

fn read_packet(input: &mut impl Iterator<Item = char>) -> Packet {
    let (version, type_id) = read_header(input);

    let mut out = Packet {
        size: 6,
        literal: None,
        header: PacketHeader { version, type_id },
        sub_packets: vec![],
    };

    match type_id {
        // Literal
        4 => {
            let (literal, read) = read_literal(input);

            out.size += read;
            out.literal = Some(literal);
        }

        // Operation
        _ => {
            let length_type_id = input.next().unwrap();
            out.size += 1;

            match length_type_id {
                '0' => {
                    let mut trailing_packet_size =
                        u64::from_str_radix(&input.take(15).collect::<String>(), 2).unwrap();
                    out.size += 15;

                    while trailing_packet_size > 0 {
                        let packet = read_packet(input);

                        trailing_packet_size -= packet.size;
                        out.size += packet.size;
                        out.sub_packets.push(packet);
                    }
                }

                '1' => {
                    let trailing_packet_count =
                        usize::from_str_radix(&input.take(11).collect::<String>(), 2).unwrap();
                    out.size += 11;

                    for _ in 0..trailing_packet_count {
                        let packet = read_packet(input);
                        out.size += packet.size;
                        out.sub_packets.push(packet);
                    }
                }

                _ => unreachable!(),
            }
        }
    };
    out
}

fn solution(input: String) -> u64 {
    let mut input = input.chars();
    let packet = read_packet(&mut input);

    packet.value()
}

fn main() {
    for input in INPUTS {
        let input = parse_input(input);
        let result = solution(input);
        println!("Result = {}", result);
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    let input = parse_input(INPUTS[1]);
    b.iter(|| {
        let result = solution(input.clone());
        test::black_box(result);
    })
}
