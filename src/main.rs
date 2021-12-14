#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

use std::collections::HashMap;
fn parse_input(input: &'static str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut input = input.split("\n\n");

    let template = input.next().unwrap().chars().collect();

    let folds: HashMap<(char, char), char> = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (s, d) = line.split_once(" -> ").unwrap();
            let mut ss = s.chars();
            let p = ss.next().unwrap();
            let q = ss.next().unwrap();

            ((p, q), d.chars().next().unwrap())
        })
        .collect();

    (template, folds)
}

fn solution((template, pairs): (Vec<char>, HashMap<(char, char), char>)) -> u64 {
    let mut map = [0; 26];
    let mut memo = HashMap::new();

    for i in 0..template.len() - 1 {
        let a = template[i];
        let b = template[i + 1];

        let out = recurse(&pairs, &mut memo, a, b, 40);

        for (a, b) in map.iter_mut().zip(out.into_iter()) {
            *a += b;
        }
        if i != template.len() - 2 {
            map[b as usize - b'A' as usize] -= 1;
        }
    }

    let mut most_common = 0;
    let mut least_common = std::u64::MAX;

    for a in map {
        most_common = std::cmp::max(most_common, a);
        if a != 0 {
            least_common = std::cmp::min(least_common, a);
        }
    }

    most_common - least_common
}

fn recurse(
    pairs: &HashMap<(char, char), char>,
    memo: &mut HashMap<(char, char, i32), [u64; 26]>,
    a: char,
    b: char,
    steps: i32,
) -> [u64; 26] {
    if steps == 0 {
        let mut out = [0; 26];
        out[a as usize - b'A' as usize] += 1;
        out[b as usize - b'A' as usize] += 1;
        return out;
    }
    if steps < 0 {
        return [0; 26];
    }

    if let Some(v) = memo.get(&(a, b, steps)) {
        return *v;
    }

    if let Some(&token) = pairs.get(&(a, b)) {
        let mut out1 = recurse(pairs, memo, a, token, steps - 1);
        let out2 = recurse(pairs, memo, token, b, steps - 1);

        for (a, b) in out1.iter_mut().zip(out2.into_iter()) {
            *a += b;
        }

        out1[token as usize - b'A' as usize] -= 1;

        memo.insert((a, b, steps), out1);
        return out1;
    }
    let mut out = [0; 26];
    out[a as usize - b'A' as usize] += 1;
    out[b as usize - b'A' as usize] += 1;
    out
}

fn main() {
    for input in INPUTS {
        let input = parse_input(input);
        let result = solution(input);
        println!("Result = {}", result);
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
