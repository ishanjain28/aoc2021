#![feature(test)]
use std::{cmp::Reverse, collections::BinaryHeap};

extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input(input: &'static str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|x| x - b'0').collect())
        .collect()
}

fn solution(grid: Vec<Vec<u8>>) -> u64 {
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![false; n]; m];

    let mut heap = BinaryHeap::with_capacity(m * n);
    heap.push((Reverse(0), 0i32, 0i32));

    while let Some((Reverse(cost), x, y)) = heap.pop() {
        if visited[x as usize][y as usize] {
            continue;
        } else {
            visited[x as usize][y as usize] = true;
        }

        if x == m as i32 - 1 && y == n as i32 - 1 {
            return cost;
        }

        for (i, j) in [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)] {
            let px = i + x;
            let py = j + y;

            if px < 0
                || py < 0
                || px >= m as i32
                || py >= n as i32
                || visited[px as usize][py as usize]
            {
                continue;
            }

            let mapped_value = map_coords(&grid, px as usize, py as usize, m, n);
            heap.push((Reverse(cost + mapped_value as u64), px, py));
        }
    }

    0
}

fn map_coords(grid: &Vec<Vec<u8>>, x: usize, y: usize, w: usize, h: usize) -> u8 {
    let gx = (x / w) as u8;
    let gy = (y / h) as u8;
    let (mx, my) = (x % w, y % h);

    (grid[mx][my] + gx + gy - 1) % 9 + 1
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
