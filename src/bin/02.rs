use std::io::{BufRead, BufReader};
use std::fs::File;

fn part1<R: BufRead>(reader: R) -> i64 {
    0
}

fn part2<R: BufRead>(reader: R) -> i64 {
    0
}

const INPUT_PATH: &str = "input/02.txt";

#[cfg(test)]
const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[test]
fn part1_test() {
    assert_eq!(part1(BufReader::new(TEST_INPUT.as_bytes())), 1);
}

#[test]
fn part2_test() {
    assert_eq!(part2(BufReader::new(TEST_INPUT.as_bytes())), 1);
}

fn main() {
    println!("Part 1");
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("{}", part1(f));

    println!("Part 2");
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("{}", part2(f));
}
