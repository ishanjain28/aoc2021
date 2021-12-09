#![feature(test)]
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

fn solution(input: Vec<Vec<u8>>) -> i32 {
    let mut answer = 0;

    let m = input.len();
    let n = input[0].len();

    for i in 0..m {
        for j in 0..n {
            let mut is_minimum = true;

            for &neighbour in NEIGHBOURS.iter() {
                let x = i as i32 + neighbour[0];
                let y = j as i32 + neighbour[1];

                if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                    continue;
                }

                if input[i][j] >= input[x as usize][y as usize] {
                    is_minimum = false;
                }
            }

            if is_minimum {
                answer += input[i][j] as i32 + 1;
            }
        }
    }

    answer
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
