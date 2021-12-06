#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input(input: &'static str) -> Vec<u8> {
    input
        .split(',')
        .filter(|&x| !x.is_empty())
        .map(|x| x.trim())
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}

fn solution(fishes: Vec<u8>) -> u64 {
    let mut day_stat = [0u64; 9];

    for fish in fishes {
        day_stat[fish as usize] += 1;
    }

    for _ in 0..80 {
        day_stat.rotate_left(1);

        day_stat[6] += day_stat[8];
    }

    day_stat.into_iter().sum::<u64>()
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
    let input = parse_input(INPUTS[1]);
    b.iter(|| {
        let result = solution(input.clone());
        test::black_box(result);
    })
}
