use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;
use std::ops::Range;
use std::collections::VecDeque;

use regex::Regex;

type Mul = (i64, i64);

struct MulPos {
    mul: Mul,
    pos: usize,
}

fn mulposes(input: &String) -> Vec<MulPos> {
    let rgx: regex::Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    rgx.captures_iter(input)
        .map(|cap| MulPos{
        pos: cap.get(0).unwrap().start(),
        mul: (
        i64::from_str(cap.get(1).unwrap().as_str()).unwrap(),
        i64::from_str(cap.get(2).unwrap().as_str()).unwrap()),
        }).collect()
}

fn muls(input: &String) -> Vec<Mul> {
    mulposes(input).into_iter().map(|mp| mp.mul).collect()
}

fn enabled_ranges(input: &String) -> Vec<Range<usize>> {
    let on_rgx = Regex::new(r"do\(\)").unwrap();
    let off_rgx = Regex::new(r"don't\(\)").unwrap();
    let mut on_pos: VecDeque<usize> = VecDeque::new();
    on_pos.push_back(0);
    let mut off_pos: VecDeque<usize> = VecDeque::new();
    for matc in on_rgx.find_iter(input.as_str()) {
        on_pos.push_back(matc.start());
    }
    for matc in off_rgx.find_iter(input.as_str()) {
        off_pos.push_back(matc.start());
    }
    off_pos.push_back(input.len());
    let mut ranges = Vec::new();
    while on_pos.len() > 0 && off_pos.len() > 0 {
        let on = on_pos.pop_front().unwrap();
        let mut off = 0;

        // consume off values that are before the current on
        while off <= on {
            off = off_pos.pop_front().unwrap();
        }

        // consume on values that are before the found off
        while let Some(nxt) = on_pos.front() {
            match nxt.cmp(&off) {
                std::cmp::Ordering::Less => {
                    on_pos.pop_front();
                },
                _ => { break; }
            }
        }

        ranges.push(on..off);
    }
    ranges
}

fn part1<R: BufRead>(mut reader: R) -> i64 {
    let mut acc: i64 = 0;
    let mut input = Vec::<u8>::new();
    reader.read_to_end(&mut input).expect("couldnt read to string");
    let input = String::from_utf8(input).expect("not utf-8");
    for (a, b) in muls(&input) {
        acc += a*b;
    }

    acc
}

fn part2<R: BufRead>(mut reader: R) -> i64 {
    let mut acc: i64 = 0;
    let mut input = Vec::<u8>::new();
    reader.read_to_end(&mut input).expect("couldnt read to string");
    let input = String::from_utf8(input).expect("not utf-8");
    let enabled = enabled_ranges(&input);
    for MulPos { pos, mul: (a, b) } in mulposes(&input) {
        if enabled.clone().into_iter().any(|r| r.contains(&pos)) {
            acc += a * b
        }
        
    }
    acc
}

const INPUT_PATH: &str = "input/03.txt";

#[cfg(test)]
const TEST_INPUT: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

#[cfg(test)]
const TEST2_INPUT: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn part1_test() {
    assert_eq!(part1(BufReader::new(TEST_INPUT.as_bytes())), 161);
}

#[test]
fn part2_test() {
    assert_eq!(part2(BufReader::new(TEST2_INPUT.as_bytes())), 48);
}

fn main() {
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("Part 1: {}", part1(f));

    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("Part 2: {}", part2(f));
}
