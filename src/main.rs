#![feature(test)]

use std::collections::HashMap;
use std::hash::Hash;
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

#[derive(Copy, Clone, PartialOrd, Ord, Hash, Debug, Eq, PartialEq)]
pub enum Segment {
    Top = 0b1000000,
    TopLeft = 0b0100000,
    TopRight = 0b0010000,
    Middle = 0b0001000,
    BottomLeft = 0b000100,
    BottomRight = 0b000010,
    Bottom = 0b0000001,
    None = 0,
}

const fn map_to_segment(c: char) -> Segment {
    match c {
        'a' => Segment::Top,
        'b' => Segment::TopLeft,
        'c' => Segment::TopRight,
        'd' => Segment::Middle,
        'e' => Segment::BottomLeft,
        'f' => Segment::BottomRight,
        'g' => Segment::Bottom,

        _ => unreachable!(),
    }
}
#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq)]
struct Display {
    layout: [Segment; 7],
}

impl Display {
    #[inline]
    const fn hash(&self) -> u8 {
        let mut out = 0;

        out |= self.layout[0] as u8;
        out |= self.layout[1] as u8;
        out |= self.layout[2] as u8;
        out |= self.layout[3] as u8;
        out |= self.layout[4] as u8;
        out |= self.layout[5] as u8;
        out |= self.layout[6] as u8;

        out
    }

    #[inline]
    const fn translate_hash_from_original(&self, to_translate: u8) -> u8 {
        let mut out = 0;

        if self.layout[0] as u8 & to_translate > 0 {
            out |= 1 << 6;
        }
        if self.layout[1] as u8 & to_translate > 0 {
            out |= 1 << 5;
        }
        if self.layout[2] as u8 & to_translate > 0 {
            out |= 1 << 4;
        }
        if self.layout[3] as u8 & to_translate > 0 {
            out |= 1 << 3;
        }
        if self.layout[4] as u8 & to_translate > 0 {
            out |= 1 << 2;
        }
        if self.layout[5] as u8 & to_translate > 0 {
            out |= 1 << 1;
        }
        if self.layout[6] as u8 & to_translate > 0 {
            out |= 1 << 0;
        }

        out
    }

    #[inline]
    const fn derive_hash_for(&self, ip: u8) -> u8 {
        let mut out = 0;
        match ip {
            0 => {
                out |= self.layout[0] as u8;
                out |= self.layout[1] as u8;
                out |= self.layout[2] as u8;
                out |= self.layout[4] as u8;
                out |= self.layout[5] as u8;
                out |= self.layout[6] as u8;
            }
            1 => {
                out |= self.layout[2] as u8;
                out |= self.layout[5] as u8;
            }
            2 => {
                out |= self.layout[0] as u8;
                out |= self.layout[2] as u8;
                out |= self.layout[3] as u8;
                out |= self.layout[4] as u8;
                out |= self.layout[6] as u8;
            }
            3 => {
                out |= self.layout[0] as u8;
                out |= self.layout[2] as u8;
                out |= self.layout[3] as u8;
                out |= self.layout[5] as u8;
                out |= self.layout[6] as u8;
            }
            4 => {
                out |= self.layout[1] as u8;
                out |= self.layout[2] as u8;
                out |= self.layout[3] as u8;
                out |= self.layout[5] as u8;
            }
            5 => {
                out |= self.layout[0] as u8;
                out |= self.layout[1] as u8;
                out |= self.layout[3] as u8;
                out |= self.layout[5] as u8;
                out |= self.layout[6] as u8;
            }
            6 => {
                out |= self.layout[0] as u8;
                out |= self.layout[1] as u8;
                out |= self.layout[3] as u8;
                out |= self.layout[4] as u8;
                out |= self.layout[5] as u8;
                out |= self.layout[6] as u8;
            }
            7 => {
                out |= self.layout[0] as u8;
                out |= self.layout[2] as u8;
                out |= self.layout[5] as u8;
            }
            8 => {
                out |= self.layout[0] as u8;
                out |= self.layout[1] as u8;
                out |= self.layout[2] as u8;
                out |= self.layout[3] as u8;
                out |= self.layout[4] as u8;
                out |= self.layout[5] as u8;
                out |= self.layout[6] as u8;
            }
            9 => {
                out |= self.layout[0] as u8;
                out |= self.layout[1] as u8;
                out |= self.layout[2] as u8;
                out |= self.layout[3] as u8;
                out |= self.layout[5] as u8;
                out |= self.layout[6] as u8;
            }
            _ => unreachable!(),
        };
        out
    }
}

impl From<&str> for Display {
    fn from(s: &str) -> Self {
        let mut layout = [Segment::None; 7];
        for (i, c) in s.chars().enumerate() {
            let to_push = match c {
                'a' => Segment::Top,
                'b' => Segment::TopLeft,
                'c' => Segment::TopRight,
                'd' => Segment::Middle,
                'e' => Segment::BottomLeft,
                'f' => Segment::BottomRight,
                'g' => Segment::Bottom,

                _ => unreachable!(),
            };

            layout[i] = to_push;
        }

        Self { layout }
    }
}

fn parse_digits(ip: &str) -> Vec<Segment> {
    let mut out = Vec::with_capacity(ip.len());

    for c in ip.chars() {
        out.push(map_to_segment(c));
    }

    out.sort_unstable();

    out
}

fn parse_input(input: &'static str) -> Vec<(Vec<Vec<Segment>>, Vec<Vec<Segment>>)> {
    input
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|line| {
            let (hints, displays) = line.split_once(" | ").unwrap();

            let hints = hints
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(parse_digits)
                .collect();
            let digits = displays
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(parse_digits)
                .collect();

            (hints, digits)
        })
        .collect()
}

pub fn permute(mut nums: [Segment; 7]) -> Vec<[Segment; 7]> {
    let mut ans = vec![];
    let k = nums.len();
    permute_helper(&mut nums, &mut ans, k);

    ans
}

fn permute_helper(nums: &mut [Segment; 7], permutations: &mut Vec<[Segment; 7]>, k: usize) {
    if k == 1 {
        permutations.push(*nums);
    } else {
        for i in 0..k {
            permute_helper(nums, permutations, k - 1);

            if i < k - 1 {
                if k % 2 == 0 {
                    nums.swap(i, k - 1);
                } else {
                    nums.swap(0, k - 1);
                }
            }
        }
    }
}

fn generate_charset(permutation: &[Segment]) -> Vec<Vec<Segment>> {
    let mut answer = vec![vec![]; 10];

    for digit in 0..=9 {
        let mut out = vec![];

        match digit {
            0 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[2]);
                out.push(permutation[4]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            1 => {
                out.push(permutation[2]);
                out.push(permutation[5]);
            }
            2 => {
                out.push(permutation[0]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[4]);
                out.push(permutation[6]);
            }
            3 => {
                out.push(permutation[0]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            4 => {
                out.push(permutation[1]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[5]);
            }
            5 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[3]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            6 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[3]);
                out.push(permutation[4]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            7 => {
                out.push(permutation[0]);
                out.push(permutation[2]);
                out.push(permutation[5]);
            }
            8 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[4]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            9 => {
                out.push(permutation[0]);
                out.push(permutation[1]);
                out.push(permutation[2]);
                out.push(permutation[3]);
                out.push(permutation[5]);
                out.push(permutation[6]);
            }
            _ => (),
        }

        answer[digit] = out;
    }

    answer
        .into_iter()
        .map(|mut set| {
            set.sort_unstable();
            set
        })
        .collect()
}

#[inline]
fn verify_perfect_overlap(d1: &[Segment], d2: &[Segment]) -> bool {
    if d1.len() != d2.len() {
        return false;
    }

    d1 == d2
}

fn find_best_fit(
    permutations: &[[Segment; 7]],
    mut hints: Vec<Vec<Segment>>,
) -> Option<Vec<Vec<Segment>>> {
    hints.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

    for permutation in permutations {
        let charset = generate_charset(permutation);
        let mut map = HashMap::new();

        for (i, cset) in charset.iter().enumerate() {
            for hint in hints.iter() {
                if verify_perfect_overlap(&cset, &hint) {
                    if map.contains_key(&hint) {
                        continue;
                    } else {
                        map.insert(hint, i);
                    }
                }
            }
        }

        if map.len() == hints.len() {
            return Some(charset);
        }
    }

    None
}

fn solution(input: Vec<(Vec<Vec<Segment>>, Vec<Vec<Segment>>)>) -> u64 {
    // Generate all possible permutations of abcdefg
    let permutations = permute([
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    ]);

    let mut answer = 0;
    for (hints, digits) in input {
        let best_fit = find_best_fit(&permutations, hints).unwrap();

        let mut out_digit = String::new();
        for mut digit in digits {
            digit.sort_unstable();

            for (i, fit) in best_fit.iter().enumerate() {
                if fit == &digit {
                    out_digit.push_str(&i.to_string());
                }
            }
        }

        let parsed = u64::from_str_radix(&out_digit, 10).unwrap();
        answer += parsed;
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
