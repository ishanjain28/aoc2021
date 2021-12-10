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
    let mut score = 0;
    for line in input {
        let mut stack = Vec::with_capacity(line.len());

        for c in line.chars() {
            match c {
                '{' | '(' | '<' | '[' => stack.push(c),
                '}' => {
                    if stack.pop() != Some('{') {
                        score += 1197
                    }
                }

                ')' => {
                    if stack.pop() != Some('(') {
                        score += 3
                    }
                }

                '>' => {
                    if stack.pop() != Some('<') {
                        score += 25137
                    }
                }

                ']' => {
                    if stack.pop() != Some('[') {
                        score += 57
                    }
                }

                _ => unreachable!(),
            }
        }
    }

    score
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
