use std::char;
use std::fs::File;
use std::io::{read_to_string, BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
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
					if line.len() > 0 {
						match lines.last() {
							Some(other) => {
								assert!(other.len() == line.len())
							}
							_ => {}
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
		if x < 0
			|| y < 0 || x >= self.width().try_into().unwrap_or_default()
			|| y >= self.height().try_into().unwrap_or_default()
		{
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
			if match wordsearch.get(x, y) {
				Some(Letter::X) => true,
				_ => false,
			} {
				for direction in directions {
					if match wordsearch.get(x + direction.0, y + direction.1) {
						Some(Letter::M) => true,
						_ => false,
					} && match wordsearch.get(x + 2 * direction.0, y + 2 * direction.1) {
						Some(Letter::A) => true,
						_ => false,
					} && match wordsearch.get(x + 3 * direction.0, y + 3 * direction.1) {
						Some(Letter::S) => true,
						_ => false,
					} {
						count += 1;
					}
				}
			}
		}
	}
	count
}

fn part2<R: BufRead>(_reader: R) -> i32 {
	9
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
	assert_eq!(part2(BufReader::new(TEST_INPUT.as_bytes())), 0);
}

fn main() {
	let f = BufReader::new(File::open(INPUT_PATH).unwrap());
	println!("Part 1: {}", part1(f));

	let f = BufReader::new(File::open(INPUT_PATH).unwrap());
	println!("Part 2: {}", part2(f));
}
