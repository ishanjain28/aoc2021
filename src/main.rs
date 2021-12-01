#![feature(test)]
extern crate test;

const INPUT: &'static str = include_str!("../inputs/day1.txt");

fn depths() -> Vec<u64> {
    INPUT
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn solution(depths: &[u64]) -> i32 {
    let mut count = 0;

    // For a window size of 3, We can consider a window size of 4
    // A + B + C && B + C + D
    // Since B, C are shared between the two consecutive sets,
    // We only need to compare A and D
    for set in depths.windows(4) {
        if set[0] < set[3] {
            count += 1;
        }
    }

    count
}

fn main() {
    let depths = &depths();
    let count = solution(depths);
    println!("increased {} times", count);
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    let depths = depths();

    b.iter(|| {
        let v = solution(&depths);
        test::black_box(v);
    })
}

#[test]
fn solution_test() {
    assert_eq!(solution(&depths()), 1429);
}
