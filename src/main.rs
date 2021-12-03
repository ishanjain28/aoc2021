#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/day3.sample.txt"),
    include_str!("../inputs/day3.txt"),
];

fn cmds(input: &'static str) -> Vec<u64> {
    input
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|x| u64::from_str_radix(x, 2).unwrap())
        .collect()
}

fn solution<const MAX_LINE_LENGTH: usize>(levels: &[u64]) -> i64 {
    let dominant_mask = count_bits::<MAX_LINE_LENGTH>(&levels, true);

    let mut gamma = 0;
    for i in (0..MAX_LINE_LENGTH).rev() {
        let v = dominant_mask & (1 << i);
        gamma <<= 1;

        if v > 0 {
            gamma |= 1;
        } else {
            gamma |= 0
        }
    }

    let epsilon = !gamma & ((1 << MAX_LINE_LENGTH as i64) - 1);
    gamma * epsilon
}

fn count_bits<const LINE_LENGTH: usize>(levels: &[u64], dominant: bool) -> u64 {
    let n = levels.len();
    let mut count = [0; LINE_LENGTH];

    for &level in levels.iter() {
        for i in 0..LINE_LENGTH {
            let c = level & (1 << i);

            if c != 0 {
                count[i as usize] += 1;
            }
        }
    }

    let mut mask = 0;
    for (i, c) in count.into_iter().enumerate() {
        if c >= n - c {
            mask |= 1 << i;
        }
    }

    if dominant {
        mask
    } else {
        !mask
    }
}

fn main() {
    let moves = cmds(INPUTS[0]);
    let count = solution::<5>(&moves);
    println!("Result {}", count);
    let moves = cmds(INPUTS[1]);
    let count = solution::<12>(&moves);
    println!("Result {}", count);
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    let moves = cmds(INPUTS[1]);
    b.iter(|| {
        let v = solution::<12>(&moves);
        test::black_box(v);
    })
}

#[test]
fn solution_test() {
    assert_eq!(solution::<5>(&cmds(INPUTS[0])), 198);
    assert_eq!(solution::<12>(&cmds(INPUTS[1])), 3687446);
}
