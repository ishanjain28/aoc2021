#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

const fn map_to_segment(c: char) -> u8 {
    1 << (c as u8 - b'a')
}

fn parse_input(input: &'static str) -> impl Iterator<Item = (Vec<u8>, Vec<u8>)> {
    input.lines().filter(|&x| !x.is_empty()).map(|line| {
        let (hints, displays) = line.split_once(" | ").unwrap();

        let hints = hints
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|hint| {
                let mut out = 0;
                for c in hint.chars() {
                    out |= map_to_segment(c);
                }
                out
            })
            .collect();

        let digits = displays
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|hint| {
                let mut out = 0;
                for c in hint.chars() {
                    out |= map_to_segment(c);
                }
                out
            })
            .collect();

        (hints, digits)
    })
}

fn decode_hint(hints: Vec<u8>) -> [u8; 10] {
    let mut out = [0; 10];

    for &hint in hints.iter() {
        match hint.count_ones() {
            2 => out[1] = hint,
            4 => out[4] = hint,
            3 => out[7] = hint,
            7 => out[8] = hint,

            _ => (),
        }
    }

    for hint in hints {
        let one_overlap = (hint & out[1]).count_ones();
        let four_overlap = (hint & out[4]).count_ones();

        match (hint.count_ones(), one_overlap, four_overlap) {
            (5, 1, 2) => out[2] = hint,
            (5, 2, 3) => out[3] = hint,
            (5, 1, 3) => out[5] = hint,
            (6, 1, 3) => out[6] = hint,
            (6, 2, 4) => out[9] = hint,
            (6, 2, 3) => out[0] = hint,
            _ => (),
        }
    }

    return out;
}

fn solution(input: impl Iterator<Item = (Vec<u8>, Vec<u8>)>) -> i32 {
    let mut answer = 0;
    for (hints, digits) in input {
        let decoded_hints = decode_hint(hints);
        let mut out = 0;

        let offset = digits.len() as u32;
        for (j, digit) in digits.into_iter().enumerate() {
            for (i, &hint) in decoded_hints.iter().enumerate() {
                if hint == digit {
                    out += i as i32 * 10i32.pow(offset - j as u32 - 1);
                }
            }
        }

        answer += out;
    }

    answer
}

fn main() {
    for input in INPUTS {
        let input = parse_input(input);
        let result = solution(input);
        println!("Result {}", result);
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse_input(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
