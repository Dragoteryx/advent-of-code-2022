use std::collections::HashSet;
use std::iter::repeat;

const INPUT: &str = include_str!("real_input.txt");

mod knot;
use knot::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
	Up,
	Down,
	Left,
	Right
}

fn moves(input: &str) -> impl Iterator<Item = Move> +'_ {
	input.lines().flat_map(|line| {
		match line.split_whitespace().collect::<Vec<_>>().as_slice() {
			["U", n] => repeat(Move::Up).take(n.parse().unwrap()),
			["D", n] => repeat(Move::Down).take(n.parse().unwrap()),
			["L", n] => repeat(Move::Left).take(n.parse().unwrap()),
			["R", n] => repeat(Move::Right).take(n.parse().unwrap()),
			_ => unreachable!()
		}
	})
}

fn rope_tail_unique_positions<const LEN: usize>(input: &str) -> usize {
	if LEN < 2 { LEN }
	else {
		let mut rope = [Knot::default(); LEN];
		let mut unique = HashSet::from([rope[0]]);
		for mov in moves(input) {
			rope[0].goto(mov);
			for i in 1..rope.len() {
				let a = rope[i-1];
				let b = &mut rope[i];
				b.join(a);
			}
			unique.insert(rope[LEN-1]);
		}
		unique.len()
	}
}

fn part1(input: &str) -> usize {
	rope_tail_unique_positions::<2>(input)
}

fn part2(input: &str) -> usize {
	rope_tail_unique_positions::<10>(input)
}

fn main() {
	println!("--- day 9 ---");
	println!("part 1 => {}", part1(INPUT));
	println!("part 2 => {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(part1(include_str!("test_input.txt")), 13);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(include_str!("test2_input.txt")), 36);
	}
}