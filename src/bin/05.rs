use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::read_to_string;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn split_rules_and_updates(s: &str) -> (&str, &str) {
	let i = s.find("\n\n").unwrap();
	let (rules, rest) = s.split_at(i);
	let updates = rest.trim();

	(rules, updates)
}

#[derive(Debug)]
struct Rules {
	ordering: HashSet<(u32, u32)>,
}

impl Rules {
	fn from_str(input: &str) -> Rules {
		let mut ordering = HashSet::new();
		for line in input.lines() {
			let pages: Vec<u32> = line
				.split('|')
				.map(|i| u32::from_str(i).expect("number"))
				.collect();
			assert_eq!(pages.len(), 2);
			let left = pages.first().unwrap().clone();
			let right = pages.last().unwrap().clone();
			ordering.insert((left, right));
		}
		Rules { ordering }
	}
}

#[derive(Debug)]
struct Update {
	pages: Vec<u32>,
}

impl Update {
	fn from_str(input: &str) -> Option<Update> {
		let pages: Vec<u32> = input
			.split(',')
			.map(|page| u32::from_str(page).expect("number"))
			.collect();
		match pages.len() % 2 {
			1 => Some(Update { pages }),
			_ => None,
		}
	}
	fn middle(&self) -> &u32 {
		self.pages.get(self.pages.len() / 2).unwrap()
	}

	fn valid(&self, rules: &Rules) -> bool {
		let mut before: Vec<u32> = Vec::new();
		for page in &self.pages {
			for previous in &before {
				if rules.ordering.contains(&(*page, *previous)) {
					return false;
				}
			}
			before.push(*page)
		}
		true
	}

	fn reorder(self, rules: &Rules) -> Self {
		// this suuuuucks
		let mut left: VecDeque<u32> = VecDeque::with_capacity(self.pages.len());
		let mut right: VecDeque<u32> = self.pages.into();
		'outer: while !right.is_empty() {
			let current = right.pop_front().unwrap();
			for i in 0..left.len() {
				let other = left.get(i).unwrap().clone();
				if rules.ordering.contains(&(current, other)) {
					left.remove(i);
					right.push_front(other);
					right.push_front(current);
					continue 'outer;
				}
			}
			left.push_back(current);
		}
		Self { pages: left.into() }
	}
}

#[test]
fn split_test() {
	assert_eq!(split_rules_and_updates("1|2\n\n1,2,3\n"), ("1|2", "1,2,3"));
	assert_eq!(
		split_rules_and_updates("1|2\n3|4\n\n1,2,3\n2,1,3"),
		("1|2\n3|4", "1,2,3\n2,1,3")
	);
}

fn part1<R: BufRead>(reader: R) -> u32 {
	let input = read_to_string(reader).unwrap();
	let (rules, updates) = split_rules_and_updates(&input);
	let rules = Rules::from_str(rules);
	let updates = updates
		.lines()
		.map(|line| Update::from_str(line).expect("valid update"));
	let mut counter = 0;
	for update in updates {
		if update.valid(&rules) {
			counter += update.middle();
		}
	}
	counter
}

fn part2<R: BufRead>(reader: R) -> u32 {
	let input = read_to_string(reader).unwrap();
	let (rules, updates) = split_rules_and_updates(&input);
	let rules = Rules::from_str(rules);
	let updates = updates
		.lines()
		.map(|line| Update::from_str(line).expect("valid update"));
	let mut counter = 0;
	for update in updates {
		if !update.valid(&rules) {
			counter += dbg!(update).reorder(&rules).middle();
		}
	}
	counter
}

const INPUT_PATH: &str = "input/05.txt";

#[cfg(test)]
const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

#[test]
fn part1_test() {
	assert_eq!(part1(BufReader::new(TEST_INPUT.as_bytes())), 143);
}

#[test]
fn part2_test() {
	assert_eq!(part2(BufReader::new(TEST_INPUT.as_bytes())), 123);
}

fn main() {
	let f = BufReader::new(File::open(INPUT_PATH).unwrap());
	println!("Part 1: {}", part1(f));

	let f = BufReader::new(File::open(INPUT_PATH).unwrap());
	println!("Part 2: {}", part2(f));
}
