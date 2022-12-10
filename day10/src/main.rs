use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
	Add(isize),
	PrepareAdd,
	Noop
}

fn instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
	input.lines().flat_map(|line| {
		match line.split_whitespace().collect::<Vec<_>>().as_slice() {
			["addx", x] => [Some(Instruction::PrepareAdd), Some(Instruction::Add(x.parse().unwrap()))],
			["noop"] => [None, Some(Instruction::Noop)],
			_ => unreachable!()
		}
	}).flatten()
}

fn instructions_with_cycle(input: &str) -> impl Iterator<Item = (isize, Instruction)> + '_ {
	instructions(input).scan(0, |i, instr| { *i += 1; Some((*i, instr)) })
}

fn part1(input: &str) -> isize {
	let mut cycles = HashMap::from([
		(20, 0), (60, 0),
		(100, 0), (140, 0),
		(180, 0), (220, 0)		
	]);

	let mut x = 1;
	for (cycle, instr) in instructions_with_cycle(input) {
		if let Entry::Occupied(mut entry) = cycles.entry(cycle) {
			entry.insert(x * cycle);
		}

		if let Instruction::Add(addx) = instr {
			x += addx;
		}
	}

	cycles.values()
		.copied().sum()
}

fn part2(input: &str) {
	let mut x = 1;
	for (cycle, instr) in instructions_with_cycle(input) {
		let pixel = (cycle - 1) % 40;

		if pixel.abs_diff(x) < 2 {
			print!("█");
		} else {
			print!(" ");
		}

		if pixel == 39 {
			println!();
		}

		if let Instruction::Add(addx) = instr {
			x += addx;
		}
	}
}

fn main() {
	const INPUT: &str = include_str!("real_input.txt");

	println!("--- day 10 ---");
	println!("part 1 => {}", part1(INPUT));
	println!("part 2");
	part2(INPUT);
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 13140);
	}
}