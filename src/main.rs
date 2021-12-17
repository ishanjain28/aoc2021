#![feature(test)]

use std::ops::RangeInclusive;
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse(input: &'static str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let input = input.trim();
    let input = input.trim_start_matches("target area: ");

    let (x, y) = input.split_once(", ").unwrap();

    let x = x.trim_start_matches("x=");
    let y = y.trim_start_matches("y=");

    let (sx, ex) = x
        .split_once("..")
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unwrap();
    let (sy, ey) = y
        .split_once("..")
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unwrap();

    (sx..=ex, sy..=ey)
}

fn check(mut vx: i32, mut vy: i32, (tx, ty): (RangeInclusive<i32>, RangeInclusive<i32>)) -> bool {
    let mut sx = 0;
    let mut sy = 0;

    loop {
        if vx == 0 && sy < *ty.start() {
            return false;
        }

        if tx.contains(&sx) && ty.contains(&sy) {
            return true;
        }

        sx += vx;
        sy += vy;

        vy -= 1;
        vx -= if vx > 0 {
            1
        } else if vx < 0 {
            -1
        } else {
            0
        };
    }
}

fn solution((xr, yr): (RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    let mut count = 0;
    for x in 0..250 {
        for y in -250..=250 {
            if check(x, y, (xr.clone(), yr.clone())) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    for input in INPUTS {
        let input = parse(input);
        let result = solution(input);
        println!("Result = {}", result);
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    let input = parse(INPUTS[1]);
    b.iter(|| {
        let result = solution(input.clone());
        test::black_box(result);
    })
}
