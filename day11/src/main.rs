mod parse;

mod monke;
use monke::*;

fn monkey_business(input: &str, rounds: usize, part2: bool) -> u128 {
	let mut monkeys = parse::input(input);

	let common_divisor: u128 = monkeys.iter().map(|monke| monke.test()).product();
	for _ in 0..rounds {
		for monkey in &monkeys {
			if part2 {
				monkey.play_turn(&monkeys, |lvl| lvl % common_divisor);
			} else {
				monkey.play_turn(&monkeys, |lvl| lvl / 3);
			}
		}
	}

	monkeys.sort_by(|monke1, monke2| {
		monke2.inspected().cmp(&monke1.inspected())
	});

	monkeys.into_iter()
		.map(|monke| monke.inspected())	
		.take(2)
		.product()
}

fn part1(input: &str) -> u128 {
	monkey_business(input, 20, false)
}

fn part2(input: &str) -> u128 {
	monkey_business(input, 10_000, true)
}

const INPUT: &str = include_str!("real_input.txt");

fn main() {
	println!("--- day 11 ---");
	println!("part 1 => {}", part1(INPUT));
	println!("part 2 => {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 10605);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 2713310158);
	}
}