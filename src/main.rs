#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/day4.sample.txt"),
    include_str!("../inputs/day4.txt"),
];

const WINNING_COMBINATIONS: [[(usize, usize); 5]; 5] = [
    // Row wins
    [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
    [(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)],
    [(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)],
    [(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)],
    [(4, 0), (4, 1), (4, 2), (4, 3), (4, 4)],
];

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum BoardEntry {
    Entry(u64),
    Called(u64),
    Null,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Board {
    board: [[BoardEntry; 5]; 5],
    sum: u64,
    won: bool,
}

impl Board {
    fn mark_draw(&mut self, draw: u64) {
        for row in self.board.iter_mut() {
            for val in row.iter_mut() {
                if let BoardEntry::Entry(v) = val {
                    if *v == draw {
                        self.sum -= *v;
                        *val = BoardEntry::Called(*v);
                    }
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for set in WINNING_COMBINATIONS.iter() {
            let col_match = set
                .iter()
                .map(|&(x, y)| self.board[y][x])
                .all(|entry| matches!(entry, BoardEntry::Called(_)));

            let row_match = set
                .iter()
                .map(|&(x, y)| self.board[x][y])
                .all(|entry| matches!(entry, BoardEntry::Called(_)));

            if row_match || col_match {
                return true;
            }
        }

        false
    }
}

fn cmds(input: &'static str) -> (Vec<u64>, Vec<Board>) {
    let mut lines = input.split("\n\n").filter(|x| !x.is_empty());

    let draws: Vec<u64> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut boards = vec![];

    for block in lines {
        let mut bblock = Board {
            board: [[BoardEntry::Null; 5]; 5],
            sum: 0,
            won: false,
        };

        for (i, line) in block.split('\n').enumerate() {
            for (j, c) in line.split(' ').filter(|x| !x.is_empty()).enumerate() {
                let num = c.parse::<u64>().unwrap();
                bblock.board[i][j] = BoardEntry::Entry(num);
                bblock.sum += num;
            }
        }

        boards.push(bblock)
    }

    (draws, boards)
}

fn solution((draws, mut boards): (Vec<u64>, Vec<Board>)) -> u64 {
    let mut answer = 0;

    for draw in draws {
        for board in boards.iter_mut() {
            if board.won {
                continue;
            }
            board.mark_draw(draw);

            if board.is_winner() {
                answer = board.sum * draw;
                board.won = true;
            }
        }
    }

    answer
}

fn main() {
    for input in INPUTS {
        let input = cmds(input);
        let result = solution(input);
        println!("Result {}", result);
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    let input = cmds(INPUTS[1]);
    b.iter(|| {
        let result = solution(input.clone());
        test::black_box(result);
    })
}
