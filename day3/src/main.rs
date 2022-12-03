use std::collections::HashSet;
use itertools::Itertools;

const INPUT: &str = include_str!("real_input.txt");

fn priority(item: char) -> u64 {
	if item.is_ascii_lowercase() {
		item as u64 - 96
	} else if item.is_ascii_uppercase() {
		item as u64 - 38
	} else {
		panic!("invalid input");
	}
}

fn part1(input: &str) -> u64 {
	input.lines().map(|line| {
		let (left, right) = line.split_at(line.len() / 2);
		let items: HashSet<_> = left.chars().collect();
		let item = right.chars()
			.find(|item| items.contains(item))
			.expect("invalid input");

		priority(item)
	}).sum()
}

fn part2(input: &str) -> u64 {
	input.lines().chunks(3).into_iter().map(|mut group| {
		let elf1: HashSet<_> = group.next().expect("invalid input").chars().collect();
		let elf2: HashSet<_> = group.next().expect("invalid input").chars().collect();
		let elf3: HashSet<_> = group.next().expect("invalid input").chars().collect();
		let elf1_2: HashSet<_> = elf1.intersection(&elf2).copied().collect();
		let elf1_2_3: HashSet<_> = elf1_2.intersection(&elf3).copied().collect();

		priority(elf1_2_3.into_iter().next().expect("invalid input"))
	}).sum()
}

fn main() {
	println!("--- day 3 ---");
	println!("part 1 => {}", part1(INPUT));
	println!("part 2 => {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 157);
		assert_eq!(part2(INPUT), 70);
	}
}