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
    let (tm, tn) = (m * 5, n * 5);

    let mut visited = vec![vec![false; tn]; tm];

    let mut heap = BinaryHeap::with_capacity(tm * tn);
    heap.push((Reverse(0), 0i32, 0i32));

    while let Some((Reverse(cost), x, y)) = heap.pop() {
        if let Some(v) = visited
            .get_mut(x as usize)
            .and_then(|row| row.get_mut(y as usize))
        {
            if *v {
                continue;
            } else {
                *v = true;
            }
        }

        if x == tm as i32 - 1 && y == tn as i32 - 1 {
            return cost;
        }

        for (i, j) in [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)] {
            let px = i + x;
            let py = j + y;

            if px < 0
                || py < 0
                || px >= tm as i32
                || py >= tn as i32
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
