#![feature(test)]
extern crate test;

use bitvec::view::BitView;

type BitSlice<'a> = &'a bitvec::slice::BitSlice<bitvec::order::Msb0, u8>;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

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
    fn new(input: &'static str) -> Self {
        let out = hex::decode(input).unwrap();

        let mut bvec: BitSlice = BitView::view_bits(out.as_slice());

        Self::parse(&mut bvec)
    }

    fn read_literal(ip: &mut BitSlice) -> (u64, u64) {
        let mut out = 0;
        let mut read = 0;

        loop {
            let (word, remaining) = ip.split_at(5);
            *ip = remaining;
            read += 5;

            out = out << 4 | read_as_u8(&word[1..]) as u64;

            if !*word.first().unwrap() {
                break;
            }
        }

        (out, read)
    }

    fn read_header(ip: &mut BitSlice) -> (u8, u8) {
        (take_u8(ip, 3), take_u8(ip, 3))
    }

    fn parse(input: &mut BitSlice) -> Packet {
        let (version, type_id) = Self::read_header(input);

        let mut out = Packet {
            size: 6,
            literal: None,
            header: PacketHeader { version, type_id },
            sub_packets: vec![],
        };

        match type_id {
            // Literal
            4 => {
                let (literal, read) = Self::read_literal(input);

                out.size += read;
                out.literal = Some(literal);
            }

            // Operation
            _ => {
                let length_type_id = take_u8(input, 1);
                out.size += 1;

                match length_type_id {
                    0 => {
                        let mut trailing_packet_size = take_u64(input, 15);
                        out.size += 15;

                        while trailing_packet_size > 0 {
                            let packet = Self::parse(input);

                            trailing_packet_size -= packet.size;
                            out.size += packet.size;
                            out.sub_packets.push(packet);
                        }
                    }

                    1 => {
                        let trailing_packet_count = take_u64(input, 11);
                        out.size += 11;

                        for _ in 0..trailing_packet_count {
                            let packet = Self::parse(input);
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
}

fn take_u8(ip: &mut BitSlice, offset: usize) -> u8 {
    let (num, left) = ip.split_at(offset);
    *ip = left;

    read_as_u8(num)
}

fn take_u64(ip: &mut BitSlice, offset: usize) -> u64 {
    let (num, left) = ip.split_at(offset);
    *ip = left;

    read_as_u64(num)
}

fn read_as_u64(ip: BitSlice) -> u64 {
    let mut out = 0;

    for bit in ip {
        out = out << 1 | if *bit { 1 } else { 0 };
    }

    out
}

fn read_as_u8(ip: BitSlice) -> u8 {
    let mut out = 0;

    for bit in ip {
        out = out << 1 | if *bit { 1 } else { 0 };
    }

    out
}

fn solution(input: &'static str) -> u64 {
    let packet = Packet::new(input.trim());

    let mut stack = vec![packet];

    let mut answer = 0;
    while let Some(packet) = stack.pop() {
        answer += packet.header.version as u64;

        stack.extend(packet.sub_packets);
    }

    answer
}

fn main() {
    for input in INPUTS {
        let result = solution(input);
        println!("Result = {}", result);
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let result = solution(INPUTS[1]);
        test::black_box(result);
    })
}
