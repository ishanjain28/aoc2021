#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input(input: &'static str) -> impl Iterator<Item = &'static str> {
    input.lines()
}

fn solution(input: impl Iterator<Item = &'static str>) -> u64 {
    let mut candidates = vec![];

    'outer: for line in input {
        let mut stack = Vec::with_capacity(line.len());

        for c in line.chars() {
            match c {
                '{' | '(' | '<' | '[' => stack.push(c),
                '}' | ']' | '>' => {
                    if stack.pop() != Some((c as u8 - 2) as char) {
                        continue 'outer;
                    }
                }

                ')' => {
                    if stack.pop() != Some('(') {
                        continue 'outer;
                    }
                }

                _ => unreachable!(),
            }
        }

        let mut score = 0;
        while let Some(v) = stack.pop() {
            score *= 5;
            match v {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,

                _ => unreachable!(),
            }
        }

        candidates.push(score);
    }

    let l = candidates.len();

    candidates.select_nth_unstable(l / 2);

    candidates[candidates.len() / 2]
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
