#![feature(test)]
#![feature(const_mut_refs)]
use std::collections::BinaryHeap;
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input<const M: usize, const N: usize>(input: &'static str) -> [[u8; N]; M] {
    let mut out = [[0; N]; M];

    for (i, line) in input.lines().filter(|&x| !x.is_empty()).enumerate() {
        for (j, c) in line.bytes().map(|x| x - b'0').enumerate() {
            out[i][j] = c;
        }
    }

    out
}

const NEIGHBOURS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn solution<const M: usize, const N: usize>(mut input: [[u8; N]; M]) -> u64 {
    let mut heap = BinaryHeap::with_capacity(3);

    for i in 0..M {
        for j in 0..N {
            let mut is_minimum = true;

            for &neighbour in NEIGHBOURS.iter() {
                let x = i as i32 + neighbour[0];
                let y = j as i32 + neighbour[1];

                if x < 0 || y < 0 || x >= M as i32 || y >= N as i32 {
                    continue;
                }

                if input[i][j] >= input[x as usize][y as usize] {
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

#[inline]
const fn dfs<const M: usize, const N: usize>(grid: &mut [[u8; N]; M], px: usize, py: usize) -> u64 {
    if grid[px][py] == VISITED {
        return 0;
    } else {
        grid[px][py] = VISITED;
    }

    let mut ldepth = 0;

    let mut x = px + 1;
    let mut y = py;
    if is_valid_coord::<M, N>(x, y) && grid[x][y] < 9 && grid[x][y] != VISITED {
        ldepth += dfs(grid, x, y);
    }

    x = px;
    y = py + 1;
    if is_valid_coord::<M, N>(x, y) && grid[x][y] < 9 && grid[x][y] != VISITED {
        ldepth += dfs(grid, x, y);
    }

    x = px.wrapping_sub(1);
    y = py;
    if is_valid_coord::<M, N>(x, y) && grid[x][y] < 9 && grid[x][y] != VISITED {
        ldepth += dfs(grid, x, y);
    }

    x = px;
    y = py.wrapping_sub(1);
    if is_valid_coord::<M, N>(x, y) && grid[x][y] < 9 && grid[x][y] != VISITED {
        ldepth += dfs(grid, x, y);
    }

    ldepth + 1
}

#[inline]
const fn is_valid_coord<const M: usize, const N: usize>(x: usize, y: usize) -> bool {
    !(x >= M || y >= N)
}

fn main() {
    let input = parse_input::<5, 10>(INPUTS[0]);
    let result = solution::<5, 10>(input);
    println!("Result {}", result);

    let input = parse_input::<100, 100>(INPUTS[1]);
    let result = solution::<100, 100>(input);
    println!("Result {}", result);
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    let input = parse_input::<100, 100>(INPUTS[1]);
    b.iter(|| {
        let result = solution(input.clone());
        test::black_box(result);
    })
}
