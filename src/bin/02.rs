use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;

enum ReportDirection {
    Up,
    Down,
}

type Report = Vec<i64>;

fn parse_line(line: String) -> Report {
    line.split_whitespace().map(|s| i64::from_str(s).expect("failed parsing int")).collect()
}

fn is_safe(report: Report) -> bool {
    let a = report.clone().into_iter();
    let b = report.into_iter().skip(1);
    let pairs = a.zip(b);
    let mut direction : Option<ReportDirection> = None;
    for (l, r) in pairs {
        match direction {
            None => {
                direction = match l.cmp(&r) {
                    std::cmp::Ordering::Less => Some(ReportDirection::Up),
                    _ => Some(ReportDirection::Down),
                }
        
            },
            Some(ReportDirection::Up) => {
                if l > r {
                    return false;
                }
            },
            Some(ReportDirection::Down) => {
                if l < r {
                    return false;
                }
            },
        }

        let diff = (l-r).abs();
        if diff > 3 || diff < 1 {
            return false
        }
        
    }

    true
}

fn generate_dampened(report: Report) -> impl Iterator<Item = Report> {
    let mut dampened: Vec<Report> = Default::default();
    for i in 0..(report.len()) {
        let mut other = report.clone();
        other.remove(i);
        dampened.push(other);

    }

    dampened.into_iter()
}

fn is_safe_dampened(report: Report) -> bool {
    match is_safe(report.clone()) {
        true => true,
        false => generate_dampened(report).any(is_safe),
    }
}

fn part1<R: BufRead>(reader: R) -> i64 {
    reader.lines().map(|l| l.expect("couldn't read")).map(parse_line)
        .map(is_safe).filter(|b| *b).count().try_into().expect("number big!!")
}

fn part2<R: BufRead>(reader: R) -> i64 {
    reader.lines().map(|l| l.expect("couldn't read")).map(parse_line)
        .map(is_safe_dampened).filter(|b| *b).count().try_into().expect("number big!!")
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
    assert_eq!(part1(BufReader::new(TEST_INPUT.as_bytes())), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(BufReader::new(TEST_INPUT.as_bytes())), 4);
}

fn main() {
    println!("Part 1");
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("{}", part1(f));

    println!("Part 2");
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    println!("{}", part2(f));
}
