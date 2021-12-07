#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input(input: &'static str) -> Vec<i64> {
    input
        .split(',')
        .filter(|&x| !x.is_empty())
        .map(|x| x.trim())
        .map(|x| i64::from_str_radix(x, 10).unwrap())
        .collect()
}

#[inline]
fn compute_cost(crabs: &[i64], mid: i64) -> i64 {
    let mut cost = 0;

    for &crab in crabs {
        let distance = (crab - mid).abs();

        cost += distance * (distance + 1) / 2;
    }
    cost
}

fn solution(crab_pos: Vec<i64>) -> i64 {
    // Ternary search on crab_pos
    // Since the solution will always be somewhere in the middle of
    // the lowest and highest values present in crab_pos
    let mut low = std::i64::MAX;
    let mut high = std::i64::MIN;

    for &crab in crab_pos.iter() {
        low = std::cmp::min(low, crab);
        high = std::cmp::max(high, crab);
    }

    // Break out when the difference is less than 2 because it'll get stuck in a infinite loop
    while high - low > 2 {
        let mid1 = low + (high - low) / 3;
        let mid2 = high - (high - low) / 3;

        let cost1 = compute_cost(&crab_pos, mid1);
        let cost2 = compute_cost(&crab_pos, mid2);

        if cost1 < cost2 {
            high = mid2;
        } else {
            low = mid1;
        }
    }

    compute_cost(&crab_pos, (low + high) / 2)
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
