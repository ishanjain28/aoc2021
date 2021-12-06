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

fn solution<const N: usize>(fishes: Vec<u8>) -> u64 {
    let mut fish_stat = [0u64; 9];
    for fish in fishes {
        fish_stat[fish as usize] += 1;
    }

    // Unrolled loop to improve performance
    let blocks = N / 9;
    for _ in 0..blocks {
        fish_stat[7] += fish_stat[0];
        fish_stat[8] += fish_stat[1];
        fish_stat[0] += fish_stat[2];
        fish_stat[1] += fish_stat[3];
        fish_stat[2] += fish_stat[4];
        fish_stat[3] += fish_stat[5];
        fish_stat[4] += fish_stat[6];
        fish_stat[5] += fish_stat[7];
        fish_stat[6] += fish_stat[8];
    }
    for day in blocks * 9..N {
        fish_stat[(day + 7) % 9] += fish_stat[day % 9];
    }

    fish_stat.into_iter().sum::<u64>()
}

fn main() {
    for input in INPUTS {
        let input = parse_input(input);
        let result = solution::<256>(input);
        println!("Result {}", result);
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    let input = parse_input(INPUTS[1]);
    b.iter(|| {
        let result = solution::<256>(input.clone());
        test::black_box(result);
    })
}
