use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;
use std::hash::Hash;

fn line_to_tuple(line: String) -> (i64, i64) {
    let mut split = line.split_whitespace();
    (i64::from_str(split.next().unwrap()).unwrap(),i64::from_str(split.next().unwrap()).unwrap())
}

fn parse_lists<R: BufRead>(reader: R) -> (Vec<i64>, Vec<i64>) {
    let v = Vec::from_iter(reader.lines().map(|l| l.unwrap()).map(line_to_tuple));
    let left = v.clone().into_iter().map(|(l, _)| l).collect();
    let right = v.into_iter().map(|(_, r)| r).collect();
    (left, right)
}

fn count<T: Eq+Hash>(vec: Vec<T>) -> HashMap<T, i64> {
    let mut map : HashMap<T, i64> = Default::default();
    for item in vec {
        let entry = map.entry(item).or_insert_with(|| 0);
        *entry += 1;
    }

    map
}

fn part1<R: BufRead>(reader: R) -> i64 {
    let (mut left, mut right) = parse_lists(reader);
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter().zip(right.into_iter()).map(|(l, r)| (l-r).abs()).reduce(|acc, new| acc + new).unwrap()
}

fn part2<R: BufRead>(reader: R) -> i64 {
    let (left, right) = parse_lists(reader);
    let right_counts = count(right);
    left.into_iter().map(|l| l * right_counts.get(&l).unwrap_or(&0)).sum()
}

const INPUT_PATH: &str = "input/01.txt";

#[cfg(test)]
const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

#[test]
fn part1_test() {
    assert_eq!(part1(BufReader::new(TEST_INPUT.as_bytes())), 11);
}

#[test]
fn part2_test() {
    assert_eq!(part2(BufReader::new(TEST_INPUT.as_bytes())), 31);
}

fn main() {
    println!("Part 1");
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("{}", part1(f));

    println!("Part 2");
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("{}", part2(f));
}
