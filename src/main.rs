#![feature(test)]

use std::collections::HashMap;
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

const fn map_to_segment(c: char) -> u8 {
    1 << (c as u8 - b'a')
}

fn parse_digits(ip: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(ip.len());

    for c in ip.chars() {
        out.push(map_to_segment(c));
    }

    out.sort_unstable();

    out
}

fn parse_input(input: &'static str) -> Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)> {
    input
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|line| {
            let (hints, displays) = line.split_once(" | ").unwrap();

            let hints = hints
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(parse_digits)
                .collect();
            let digits = displays
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(parse_digits)
                .collect();

            (hints, digits)
        })
        .collect()
}

pub fn permute(mut nums: [u8; 7]) -> Vec<[u8; 7]> {
    let mut ans = vec![];
    let k = nums.len();
    permute_helper(&mut nums, &mut ans, k);

    ans
}

fn permute_helper(nums: &mut [u8; 7], permutations: &mut Vec<[u8; 7]>, k: usize) {
    if k == 1 {
        permutations.push(*nums);
    } else {
        for i in 0..k {
            permute_helper(nums, permutations, k - 1);

            if i < k - 1 {
                if k % 2 == 0 {
                    nums.swap(i, k - 1);
                } else {
                    nums.swap(0, k - 1);
                }
            }
        }
    }
}

fn generate_charset(permutation: &[u8]) -> Vec<Vec<u8>> {
    let mut answer = vec![vec![]; 10];

    for digit in 0..=9 {
        let mut out = vec![];

        match digit {
            0 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[2]);
                out.push(permutation[4]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            1 => {
                out.push(permutation[2]);
                out.push(permutation[5]);
            }
            2 => {
                out.push(permutation[0]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[4]);
                out.push(permutation[6]);
            }
            3 => {
                out.push(permutation[0]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            4 => {
                out.push(permutation[1]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[5]);
            }
            5 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[3]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            6 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[3]);
                out.push(permutation[4]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            7 => {
                out.push(permutation[0]);
                out.push(permutation[2]);
                out.push(permutation[5]);
            }
            8 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[4]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            9 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            _ => (),
        }

        answer[digit] = out;
    }

    answer
        .into_iter()
        .map(|mut set| {
            set.sort_unstable();
            set
        })
        .collect()
}

#[inline]
fn verify_perfect_overlap(d1: &[u8], d2: &[u8]) -> bool {
    if d1.len() != d2.len() {
        return false;
    }

    d1 == d2
}

fn find_best_fit(permutations: &[[u8; 7]], mut hints: Vec<Vec<u8>>) -> Option<Vec<Vec<u8>>> {
    hints.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

    for permutation in permutations {
        let charset = generate_charset(permutation);
        let mut map = HashMap::new();

        for (i, cset) in charset.iter().enumerate() {
            for hint in hints.iter() {
                if verify_perfect_overlap(&cset, &hint) {
                    if map.contains_key(&hint) {
                        continue;
                    } else {
                        map.insert(hint, i);
                    }
                }
            }
        }

        if map.len() == hints.len() {
            return Some(charset);
        }
    }

    None
}

fn solution(input: Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>) -> u64 {
    // Generate all possible permutations of abcdefg
    let permutations = permute([
        0b0000001, 0b0000010, 0b0000100, 0b0001000, 0b0010000, 0b0100000, 0b1000000,
    ]);

    let mut answer = 0;
    for (hints, digits) in input {
        let best_fit = find_best_fit(&permutations, hints).unwrap();

        let mut out_digit = String::new();
        for digit in digits {
            for (i, fit) in best_fit.iter().enumerate() {
                if fit == &digit {
                    out_digit.push_str(&i.to_string());
                }
            }
        }

        let parsed = u64::from_str_radix(&out_digit, 10).unwrap();
        answer += parsed;
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
    let input = parse_input(INPUTS[1]);
    b.iter(|| {
        let result = solution(input.clone());
        test::black_box(result);
    })
}
