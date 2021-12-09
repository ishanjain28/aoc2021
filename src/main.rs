#![feature(test)]

use std::collections::BinaryHeap;
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input(input: &'static str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|line| line.bytes().map(|x| x - b'0').collect())
        .collect()
}

const NEIGHBOURS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn solution(mut input: Vec<Vec<u8>>) -> u64 {
    let m = input.len() as i32;
    let n = input[0].len() as i32;

    let mut heap = BinaryHeap::with_capacity(3);

    for i in 0..m {
        for j in 0..n {
            let mut is_minimum = true;

            for &neighbour in NEIGHBOURS.iter() {
                let x = i + neighbour[0];
                let y = j + neighbour[1];

                if x < 0 || y < 0 || x >= m || y >= n {
                    continue;
                }

                if input[i as usize][j as usize] >= input[x as usize][y as usize] {
                    is_minimum = false;
                    break;
                }
            }

            if is_minimum {
                let basin_size = dfs(&mut input, i, j);

                heap.push(basin_size);
            }
        }
    }

    heap.into_iter().take(3).fold(1, |a, x| a * x)
}

const VISITED: u8 = 1 << 7;

fn dfs(grid: &mut [Vec<u8>], px: i32, py: i32) -> u64 {
    if grid[px as usize][py as usize] == VISITED {
        return 0;
    } else {
        grid[px as usize][py as usize] = VISITED;
    }

    let mut ldepth = 0;
    for neigh in NEIGHBOURS.iter() {
        let x = px as i32 + neigh[0];
        let y = py as i32 + neigh[1];

        if x < 0
            || y < 0
            || x >= grid.len() as i32
            || y >= grid[0].len() as i32
            || grid[x as usize][y as usize] == 9
            || grid[x as usize][y as usize] == VISITED
        {
            continue;
        }

        ldepth += dfs(grid, x, y);
    }

    ldepth + 1
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
