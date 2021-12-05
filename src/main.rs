#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

#[derive(Debug, Clone)]
struct LineSegment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl LineSegment {
    fn generate_points(&self) -> Vec<(usize, usize)> {
        if self.x1 == self.x2 {
            let ymin = std::cmp::min(self.y1, self.y2);
            let ymax = std::cmp::max(self.y1, self.y2);

            return (ymin..=ymax).map(|y| (self.x1, y)).collect();
        } else if self.y1 == self.y2 {
            let xmin = std::cmp::min(self.x1, self.x2);
            let xmax = std::cmp::max(self.x1, self.x2);

            return (xmin..=xmax).map(|x| (x, self.y1)).collect();
        } else {
            vec![]
        }
    }
}

fn parse_input(input: &'static str) -> Vec<LineSegment> {
    let lines = input.split('\n').filter(|x| !x.is_empty());
    lines
        .map(|line| {
            let mut line = line.split(" -> ");

            let start: Vec<usize> = line
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let end: Vec<usize> = line
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            LineSegment {
                x1: start[0],
                y1: start[1],
                x2: end[0],
                y2: end[1],
            }
        })
        .collect()
}

fn solution(coords: Vec<LineSegment>) -> u64 {
    let mut answer = 0;
    let mut grid = vec![vec![0; 1000]; 1000];

    for coord in coords {
        for (x, y) in coord.generate_points() {
            grid[x][y] += 1;
        }
    }

    for row in grid {
        for val in row {
            if val >= 2 {
                answer += 1;
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
