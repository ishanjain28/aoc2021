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
        let mut x = x.split_ascii_whitespace();

        let cmd = x.next();
        let val = x.next().map(|x| x.parse::<i32>()).unwrap().unwrap();

        match cmd {
            Some("forward") => Moves::Forward(val),
            Some("down") => Moves::Down(val),
            Some("up") => Moves::Up(val),
            _ => unreachable!(),
        }
    })
}

struct Ship {
    aim: i32,
    vert_pos: i32,
    hor_pos: i32,
}

fn solution(moves: impl Iterator<Item = Moves>) -> i32 {
    let mut ship = Ship {
        aim: 0,
        hor_pos: 0,
        vert_pos: 0,
    };

    for mmove in moves {
        match mmove {
            Moves::Forward(v) => {
                ship.hor_pos += v;
                ship.vert_pos += ship.aim * v;
            }
            Moves::Down(v) => ship.aim += v,
            Moves::Up(v) => ship.aim -= v,
        }
    }

    ship.hor_pos * ship.vert_pos
}

fn main() {
    let moves = cmds();
    let count = solution(moves);
    println!("increased {} times", count);
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        // Not ideal since we want to benchmark the function
        // _excluding_ the time it takes to go through the input
        let moves = cmds();
        let v = solution(moves);
        test::black_box(v);
    })
}

#[test]
fn solution_test() {
    assert_eq!(solution(cmds()), 1463827010);
}
