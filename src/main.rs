#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse_input(input: &'static str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|x| x.bytes().map(|x| (x - b'0') as u64).collect())
        .collect()
}

fn solution(mut input: Vec<Vec<u64>>) -> u64 {
    let m = input.len();
    let n = input[0].len();
    let mut step = 1;

    loop {
        for row in input.iter_mut() {
            for col in row {
                if *col == 9 {
                    *col = 0;
                } else {
                    *col += 1;
                }
            }
        }

        let mut tmp = input.clone();

        for i in 0..m {
            for j in 0..n {
                if input[i][j] == 0 {
                    dfs(&mut tmp, i as i32, j as i32);
                }
            }
        }
        input = tmp;

        if input.iter().all(|row| row.iter().all(|&c| c == 0)) {
            return step;
        }
        step += 1;
    }
}

fn dfs(ip: &mut Vec<Vec<u64>>, i: i32, j: i32) {
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            let px = i + x;
            let py = j + y;

            if px < 0
                || py < 0
                || px >= ip.len() as i32
                || py >= ip[0].len() as i32
                || ip[px as usize][py as usize] == 0
            {
                continue;
            }

            if ip[px as usize][py as usize] == 9 {
                ip[px as usize][py as usize] = 0;
            } else {
                ip[px as usize][py as usize] += 1;
            }

            if ip[px as usize][py as usize] == 0 {
                dfs(ip, px, py);
            }
        }
    }
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
    b.iter(|| {
        let input = parse_input(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
