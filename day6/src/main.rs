use itertools::Itertools;

const INPUT: &str = include_str!("real_input.txt");

fn find_marker(input: &str, size: usize) -> Option<usize> {
	let chars = input.chars().collect::<Vec<_>>();
	chars.windows(size)
		.enumerate()
		.find(|(_, chars)| chars.iter().duplicates().count() == 0)
		.map(|(n, _)| n + size)
}

fn part1(input: &str) -> Option<usize> {
	find_marker(input, 4)
}

fn part2(input: &str) -> Option<usize> {
	find_marker(input, 14)
}

fn main() {
	println!("--- day 6 ---");
	println!("part 1 => {}", part1(INPUT).unwrap());
	println!("part 2 => {}", part2(INPUT).unwrap());
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), Some(7));
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), Some(19));
	}
}