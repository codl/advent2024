use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;

use regex::Regex;

type Mul = (i64, i64);

fn muls(input: String) -> Vec<Mul> {
    let rgx: regex::Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    rgx.captures_iter(&input)
        .map(|cap| (
        i64::from_str(cap.get(1).unwrap().as_str()).unwrap(),
        i64::from_str(cap.get(2).unwrap().as_str()).unwrap()
    )).collect()
    
}

fn part1<R: BufRead>(mut reader: R) -> i64 {
    let mut acc: i64 = 0;
    let mut input = Vec::<u8>::new();
    reader.read_to_end(&mut input).expect("couldnt read to string");
    let input = String::from_utf8(input).expect("not utf-8");
    for (a, b) in muls(input) {
        acc += a*b;
    }

    acc
}

fn part2<R: BufRead>(_reader: R) -> i64 {
    0
}

const INPUT_PATH: &str = "input/03.txt";

#[cfg(test)]
const TEST_INPUT: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

#[test]
fn part1_test() {
    assert_eq!(part1(BufReader::new(TEST_INPUT.as_bytes())), 161);
}

#[test]
fn part2_test() {
    assert_eq!(part2(BufReader::new(TEST_INPUT.as_bytes())), 48);
}

fn main() {
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("Part 1: {}", part1(f));

    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("Part 2: {}", part2(f));
}
