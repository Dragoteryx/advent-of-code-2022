mod monke;
use monke::*;

/// Hardcoded because I'm lazy.
fn monkeys() -> [Monke; 8] {
	[
		Monke::new(
			vec![54, 82, 90, 88, 86, 54],
			|worry_level| worry_level * 7,
			11,
			2, 6
		),
		Monke::new(
			vec![91, 65],
			|worry_level| worry_level * 13,
			5, 7, 4
		),
		Monke::new(
			vec![62, 54, 57, 92, 83, 63, 63],
			|worry_level| worry_level + 1,
			7, 1, 7
		),
		Monke::new(
			vec![67, 72, 68],
			|worry_level| worry_level * worry_level,
			2, 0, 6
		),
		Monke::new(
			vec![68, 89, 90, 86, 84, 57, 72, 84],
			|worry_level| worry_level + 7,
			17, 3, 5
		),
		Monke::new(
			vec![79, 83, 64, 58],
			|worry_level| worry_level + 6,
			13, 3, 0
		),
		Monke::new(
			vec![96, 72, 89, 70, 88],
			|worry_level| worry_level + 4,
			3, 1, 2
		),
		Monke::new(
			vec![79],
			|worry_level| worry_level + 8,
			19, 4, 5
		)
	]
}

fn monkey_business(rounds: usize, part2: bool) -> u128 {
	let mut monkeys = monkeys();

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

fn part1() -> u128 {
	monkey_business(20, false)
}

fn part2() -> u128 {
	monkey_business(10_000, true)
}

fn main() {
	println!("--- day 11 ---");
	println!("part 1 => {}", part1());
	println!("part 2 => {}", part2());
}