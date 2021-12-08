#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Segment {
    Top = 0b0000001,
    TopLeft = 0b0000010,
    TopRight = 0b0000100,
    Middle = 0b0001000,
    BottomLeft = 0b0010000,
    BottomRight = 0b0100000,
    Bottom = 0b1000000,
    None = 0,
}

#[derive(Debug, Copy, Clone)]
struct Display {
    layout: [Segment; 7],
}

impl From<&str> for Display {
    fn from(s: &str) -> Self {
        let mut layout = [Segment::None; 7];
        for (i, c) in s.chars().enumerate() {
            let to_push = match c {
                'a' => Segment::Top,
                'b' => Segment::TopLeft,
                'c' => Segment::TopRight,
                'd' => Segment::Middle,
                'e' => Segment::BottomLeft,
                'f' => Segment::BottomRight,
                'g' => Segment::Bottom,

                _ => unreachable!(),
            };

            layout[i] = to_push;
        }

        Self { layout }
    }
}

fn parse_input(input: &'static str) -> impl Iterator<Item = (Vec<Display>, Vec<Display>)> {
    input.lines().filter(|&x| !x.is_empty()).map(|x| {
        let mut x = x.split("|");
        let hints = x
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.into())
            .collect();
        let digits = x
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.into())
            .collect();

        (hints, digits)
    })
}

fn solution(input: impl Iterator<Item = (Vec<Display>, Vec<Display>)>) -> u64 {
    let mut answer = 0;
    for (_, digits) in input {
        for digit in digits {
            let set_bits_count = digit.layout.iter().filter(|&&s| s == Segment::None).count();

            match 7 - set_bits_count {
                2 | 7 | 4 | 3 => answer += 1,
                _ => (),
            }
        }
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
