#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input(input: &'static str) -> (Vec<Vec<usize>>, Vec<bool>) {
    let mut map = HashMap::from([("start", 0), ("end", 1)]);
    let mut cap_list = vec![false; 2];
    let mut adj_list = vec![vec![]; 2];

    for (a, b) in input.lines().map(|x| x.split_once('-')).flatten() {
        let mut process = |x: &'static str| -> usize {
            let xi = if let Some(&v) = map.get(x) {
                v
            } else {
                adj_list.push(vec![]);
                cap_list.push(x.chars().any(char::is_uppercase));
                let i = map.len();
                map.insert(x, i);
                i
            };

            xi
        };

        let si = process(a);
        let di = process(b);

        // Don't add mappings with destination start or source end
        if di != 0 && si != 1 {
            adj_list[si].push(di);
        }

        if si != 0 && di != 1 {
            adj_list[di].push(si);
        }
    }

    (adj_list, cap_list)
}

use std::collections::HashMap;

fn solution((adj_list, cap_list): (Vec<Vec<usize>>, Vec<bool>)) -> u64 {
    let mut visited = vec![false; cap_list.len()];

    dfs(&adj_list, &cap_list, &mut visited, 0)
}

#[inline]
fn dfs(adj_list: &Vec<Vec<usize>>, cap_list: &[bool], visited: &mut Vec<bool>, node: usize) -> u64 {
    if node == 1 {
        return 1;
    }

    let mut paths = 0;
    for &neighbour in adj_list[node].iter() {
        if !visited[neighbour] || cap_list[neighbour] {
            visited[neighbour] = true;
            paths += dfs(adj_list, cap_list, visited, neighbour);
            visited[neighbour] = false;
        }
    }

    paths
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
