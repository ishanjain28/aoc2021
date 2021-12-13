#![feature(test)]

use std::fmt::{Display, Write};
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

#[derive(Copy, Eq, PartialEq, Clone)]
enum NodeType {
    Dot,
    Empty,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Dot => f.write_char('\u{2588}'),
            NodeType::Empty => f.write_char('.'),
        }
    }
}

impl NodeType {
    #[inline]
    const fn or(&self, rhs: &NodeType) -> NodeType {
        match (self, rhs) {
            (_, NodeType::Dot) | (NodeType::Dot, _) => NodeType::Dot,
            _ => NodeType::Empty,
        }
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

fn parse_input(input: &'static str) -> (Vec<Point>, Vec<Fold>) {
    let mut input = input.split("\n\n");

    let points = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();

            Point {
                x: usize::from_str_radix(x, 10).unwrap(),
                y: usize::from_str_radix(y, 10).unwrap(),
            }
        })
        .collect();

    let folds = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.trim_start_matches("fold along ");
            let (axis, position) = line.split_once('=').unwrap();

            let position = usize::from_str_radix(position, 10).unwrap();
            match axis {
                "x" => Fold::X(position),
                "y" => Fold::Y(position),
                _ => unreachable!(),
            }
        })
        .collect();

    (points, folds)
}

fn solution((points, folds): (Vec<Point>, Vec<Fold>)) -> Vec<Vec<NodeType>> {
    let mut y_max = 0;
    let mut x_max = 0;
    for point in points.iter() {
        y_max = std::cmp::max(y_max, point.y);
        x_max = std::cmp::max(x_max, point.x);
    }
    let mut grid = vec![vec![NodeType::Empty; x_max + 2]; y_max + 2];

    for point in points {
        grid[point.y][point.x] = NodeType::Dot
    }

    for fold in folds {
        let m = grid.len();
        let n = grid[0].len();
        match fold {
            Fold::X(position) => {
                for j in 0..m {
                    for i in (0..position).rev() {
                        let mirror = std::mem::replace(&mut grid[j][n - i - 1], NodeType::Empty);
                        grid[j][i] = grid[j][i].or(&mirror);
                    }

                    grid[j] = grid[j].iter().take(position).cloned().collect();
                }
            }
            Fold::Y(position) => {
                for i in 0..n {
                    for j in (0..position).rev() {
                        let mirror =
                            std::mem::replace(&mut grid[2 * position - j][i], NodeType::Empty);
                        grid[j][i] = grid[j][i].or(&mirror);
                    }
                }

                grid = grid.into_iter().take(position).collect();
            }
        }
    }

    grid
}

fn main() {
    for input in INPUTS {
        let input = parse_input(input);
        let grid = solution(input);
        for row in grid.iter() {
            for c in row {
                print!("{}", c);
            }
            println!("")
        }
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
