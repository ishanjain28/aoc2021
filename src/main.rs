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

fn solution<const MAX_LENGTH: usize>(levels: Vec<u64>) -> u64 {
    let oxygen_gen_rating = filter_candidates::<MAX_LENGTH>(levels.clone(), true);
    let co2_scrub_rating = filter_candidates::<MAX_LENGTH>(levels, false);

    oxygen_gen_rating * co2_scrub_rating
}

fn filter_candidates<const MAX_LENGTH: usize>(mut levels: Vec<u64>, dominant: bool) -> u64 {
    let mut i = 0;
    while levels.len() > 1 {
        let count = count_bits::<MAX_LENGTH>(&levels, dominant);
        let leading = count & (1 << (MAX_LENGTH - i - 1));

        levels = levels
            .into_iter()
            .filter(|&x| {
                let pos = x & (1 << (MAX_LENGTH - i - 1));

                return pos == leading;
            })
            .collect();

        i += 1;
    }

    levels[0]
}

fn count_bits<const MAX_LENGTH: usize>(levels: &[u64], dominant: bool) -> u64 {
    let n = levels.len();
    let mut count = [0; MAX_LENGTH];
    let mut mask = 0;

    for &level in levels.iter() {
        for i in 0..MAX_LENGTH {
            let c = level & (1 << (MAX_LENGTH - i - 1));

            if c != 0 {
                count[i as usize] += 1;
            }
        }
    }

    for (i, c) in count.into_iter().enumerate() {
        if c >= n - c {
            mask |= 1 << (MAX_LENGTH - i - 1);
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
    let count = solution::<5>(moves);
    println!("Result {}", count);
    let moves = cmds(INPUTS[1]);
    let count = solution::<12>(moves);
    println!("Result {}", count);
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    let moves = cmds(INPUTS[1]);
    b.iter(|| {
        let v = solution::<12>(moves.clone());
        test::black_box(v);
    })
}

#[test]
fn solution_test() {
    assert_eq!(solution::<5>(cmds(INPUTS[0])), 230);
    assert_eq!(solution::<12>(cmds(INPUTS[1])), 4406844);
}
