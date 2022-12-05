use std::collections::VecDeque;

const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Move {
	n: usize,
	from: usize,
	to: usize
}

fn parse_input(input: &str, n: usize) -> (Vec<VecDeque<char>>, impl Iterator<Item = Move> + '_) {
	let (boxes_str, moves_str) = input.split_once("\n\n").expect("invalid input");

	let mut boxes = vec![VecDeque::new(); n];
	for line in boxes_str.lines() {
		let mut chars = line.chars().skip(1);
		boxes.iter_mut().for_each(|b| {
			let c = chars.next().expect("invalid input");
			if c.is_alphabetic() {
				b.push_back(c);
			}

			chars.next();
			chars.next();
			chars.next();
		});
	}

	(boxes, moves_str.lines().map(|line| {
		let mut split = line.split_whitespace().skip(1);
		let n = split.next().unwrap().parse().unwrap();
		split.next();
		let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
		split.next();
		let to = split.next().unwrap().parse::<usize>().unwrap() - 1;

		Move { n, from, to }
	}))
}

fn top_boxes(boxes: &[VecDeque<char>]) -> String {
	boxes.iter().fold(String::new(), |mut s, b| {
		if let Some(c) = b.get(0) { s += &c.to_string(); }
		s
	})
}

fn part1(input: &str, n: usize) -> String {
	let (mut boxes, moves) = parse_input(input, n);
	let boxes_mut: *mut _ = &mut boxes;
	for mov in moves {
		if mov.from == mov.to { continue; }
		unsafe {
			let from = &mut (*boxes_mut)[mov.from];
			let to = &mut (*boxes_mut)[mov.to];
			for _ in 0..mov.n {
				to.push_front(from.pop_front().unwrap());
			}
		}
	}

	top_boxes(&boxes)
}

fn part2(input: &str, n: usize) -> String {
	let (mut boxes, moves) = parse_input(input, n);
	let boxes_mut: *mut _ = &mut boxes;
	for mov in moves {
		if mov.from == mov.to { continue; }
		unsafe {
			let from = &mut (*boxes_mut)[mov.from];
			let to = &mut (*boxes_mut)[mov.to];
			for c in from.drain(0..mov.n).rev() {
				to.push_front(c);
			}
		}
	}

	top_boxes(&boxes)
}

fn main() {
	println!("--- day 5 ---");
	println!("part 1 => {}", part1(INPUT, 9));
	println!("part 2 => {}", part2(INPUT, 9));
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT, 3).as_str(), "CMZ");
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT, 3).as_str(), "MCD");
	}
}