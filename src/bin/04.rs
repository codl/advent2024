use std::char;
use std::fs::File;
use std::io::{read_to_string, BufRead, BufReader};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Letter {
	X,
	M,
	A,
	S,
}

impl Letter {
	fn from_char(input: char) -> Option<Letter> {
		match input {
			'X' => Some(Letter::X),
			'M' => Some(Letter::M),
			'A' => Some(Letter::A),
			'S' => Some(Letter::S),
			_ => None,
		}
	}
}

struct Wordsearch(Vec<Vec<Letter>>);

impl Wordsearch {
	fn from_str(input: &str) -> Wordsearch {
		let mut lines: Vec<Vec<Letter>> = Vec::new();
		let mut line: Vec<Letter> = Vec::new();
		for char in input.chars() {
			match Letter::from_char(char) {
				Some(letter) => {
					line.push(letter);
				}
				None => {
					if !line.is_empty() {
						if let Some(other) = lines.last() {
							assert!(other.len() == line.len())
						}
						lines.push(line);
						line = Vec::new();
					}
				}
			}
		}
		Wordsearch(lines)
	}

	fn height(&self) -> i32 {
		self.0.len().try_into().unwrap()
	}

	fn width(&self) -> i32 {
		match self.height() {
			0 => 0,
			_ => self.0.first().unwrap().len().try_into().unwrap(),
		}
	}

	fn get(&self, x: i32, y: i32) -> Option<Letter> {
		if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
			return None;
		}
		let x: usize = x.try_into().unwrap();
		let y: usize = y.try_into().unwrap();
		Some(*self.0.get(y).unwrap().get(x).unwrap())
	}
}

fn part1<R: BufRead>(reader: R) -> i32 {
	let input = read_to_string(reader).unwrap();
	let wordsearch = Wordsearch::from_str(&input);
	let directions: [(i32, i32); 8] = [
		(0, 1),
		(1, 1),
		(1, 0),
		(1, -1),
		(0, -1),
		(-1, -1),
		(-1, 0),
		(-1, 1),
	];
	let mut count = 0;
	for x in 0..wordsearch.width() {
		for y in 0..wordsearch.height() {
			if matches!(wordsearch.get(x, y), Some(Letter::X)) {
				for direction in directions {
					if matches!(
						wordsearch.get(x + direction.0, y + direction.1),
						Some(Letter::M)
					) && matches!(
						wordsearch.get(x + 2 * direction.0, y + 2 * direction.1),
						Some(Letter::A)
					) && matches!(
						wordsearch.get(x + 3 * direction.0, y + 3 * direction.1),
						Some(Letter::S)
					) {
						count += 1;
					}
				}
			}
		}
	}
	count
}

fn part2<R: BufRead>(reader: R) -> i32 {
	let input = read_to_string(reader).unwrap();
	let wordsearch = Wordsearch::from_str(&input);
	let directions: [(i32, i32); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
	let mut count = 0;
	for x in 0..wordsearch.width() {
		for y in 0..wordsearch.height() {
			if matches!(wordsearch.get(x, y), Some(Letter::A)) {
				let arms: Vec<Option<Letter>> = directions
					.iter()
					.map(|(xx, yy)| wordsearch.get(x + xx, y + yy))
					.collect();
				if arms.iter().any(|arm| arm.is_none()) {
					continue;
				}
				let arms: Vec<Letter> = arms.iter().map(|o| o.unwrap()).collect();
				if arms
					.iter()
					.any(|arm| matches!(arm, Letter::X) || matches!(arm, Letter::A))
				{
					continue;
				}
				if arms.get(0) == arms.get(2) || arms.get(1) == arms.get(3) {
					continue;
				}
				count += 1;
			}
		}
	}
	count
}

const INPUT_PATH: &str = "input/04.txt";

#[cfg(test)]
const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[test]
fn part1_test() {
	assert_eq!(part1(BufReader::new(TEST_INPUT.as_bytes())), 18);
}

#[test]
fn part2_test() {
	assert_eq!(part2(BufReader::new(TEST_INPUT.as_bytes())), 9);
}

fn main() {
	let f = BufReader::new(File::open(INPUT_PATH).unwrap());
	println!("Part 1: {}", part1(f));

	let f = BufReader::new(File::open(INPUT_PATH).unwrap());
	println!("Part 2: {}", part2(f));
}
