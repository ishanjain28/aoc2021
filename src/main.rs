#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/day4.sample.txt"),
    include_str!("../inputs/day4.txt"),
];

#[derive(Copy, Clone, Debug)]
enum BoardEntry {
    Entry(u64),
    CalledEntry(u64),
    Null,
}

#[derive(Copy, Clone, Debug)]
struct Board {
    board: [[BoardEntry; 5]; 5],
    sum: u64,
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
    for draw in draws {
        // Mark boards

        mark_entry(&mut boards, draw);

        if let Some(board) = check_winner(&boards) {
            return board.sum * draw;
        }
    }

    0
}

fn check_winner(boards: &[Board]) -> Option<&Board> {
    for board in boards {
        let grid = board.board;

        for i in 0..5 {
            if grid[i]
                .iter()
                .all(|x| matches!(x, BoardEntry::CalledEntry(_)))
            {
                return Some(board);
            }
        }

        for j in 0..5 {
            let mut all_called = true;
            for i in 0..5 {
                if !matches!(grid[i][j], BoardEntry::CalledEntry(_)) {
                    all_called = false;
                    break;
                }
            }

            if all_called {
                return Some(board);
            }
        }
    }

    None
}

fn mark_entry(boards: &mut [Board], called: u64) {
    for board in boards {
        for row in board.board.iter_mut() {
            for col in row.iter_mut() {
                if let BoardEntry::Entry(val) = col {
                    if *val == called {
                        board.sum -= *val;
                        *col = BoardEntry::CalledEntry(*val);
                    }
                }
            }
        }
    }
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
    b.iter(|| {
        let input = cmds(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
