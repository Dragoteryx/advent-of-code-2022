const INPUT: &str = include_str!("real_input.txt");

fn pairs(input: &str) -> impl Iterator<Item = ((u8, u8), (u8, u8))> +'_ {
	input.lines().map(|line| {
		let (left, right) = line.split_once(',').expect("invalid input");
		let (left_a, left_b) = left.split_once('-').expect("invalid input");
		let (right_a, right_b) = right.split_once('-').expect("invalid input");

		(
			(left_a.parse().unwrap(), left_b.parse().unwrap()),
			(right_a.parse().unwrap(), right_b.parse().unwrap())
		)
	})
}

fn part1(input: &str) -> usize {
	pairs(input).filter(|(left, right)| {
		(left.0 <= right.0 && left.1 >= right.1)
		|| (right.0 <= left.0 && right.1 >= left.1)
	}).count()
}

fn part2(input: &str) -> usize {
	pairs(input).filter(|(left, right)| {
		left.0 <= right.1 && right.0 <= left.1
	}).count()
}

fn main() {
	println!("--- day 4 ---");
	println!("part 1 => {}", part1(INPUT));
	println!("part 2 => {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 2);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 4);
	}

}