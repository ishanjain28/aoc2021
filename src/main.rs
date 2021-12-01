const INPUT: &'static str = include_str!("../inputs/day1.txt");

fn depths() -> Vec<u64> {
    INPUT
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn main() {
    let days = depths();
    let mut count = 0;

    for set in days.windows(2) {
        if set[1] > set[0] {
            count += 1;
        }
    }

    println!("increased {} times", count);
}
