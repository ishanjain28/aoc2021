#![feature(test)]
extern crate test;

const INPUT: &'static str = include_str!("../inputs/day2.txt");

enum Moves {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn cmds() -> impl Iterator<Item = Moves> {
    INPUT.lines().filter(|&x| !x.is_empty()).map(|x| {
        let x: Vec<&str> = x.split_ascii_whitespace().collect();

        match x[0] {
            "forward" => Moves::Forward(x[1].parse::<i32>().unwrap()),
            "down" => Moves::Down(x[1].parse::<i32>().unwrap()),
            "up" => Moves::Up(x[1].parse::<i32>().unwrap()),
            _ => unreachable!(),
        }
    })
}

fn solution(moves: impl Iterator<Item = Moves>) -> i32 {
    let mut vertical = 0;
    let mut horizontal = 0;

    for mmove in moves {
        match mmove {
            Moves::Forward(v) => horizontal += v,
            Moves::Down(v) => vertical += v,
            Moves::Up(v) => vertical -= v,
        }
    }

    vertical * horizontal
}

fn main() {
    let moves = cmds();
    let count = solution(moves);
    println!("increased {} times", count);
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let moves = cmds();
        let v = solution(moves);
        test::black_box(v);
    })
}

#[test]
fn solution_test() {
    assert_eq!(solution(cmds()), 1429);
}
