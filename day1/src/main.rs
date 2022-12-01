const INPUT: &str = include_str!("real_input.txt");

fn calories(input: &str) -> impl Iterator<Item = u64> + '_ {
	let mut input = input.lines();
	std::iter::from_fn(move || {
		let mut sum = 0;
		for line in input.by_ref() {
			if let Ok(n) = line.parse::<u64>() {
				sum += n;
			} else {
				break;
			}
		}

		if sum > 0 {
			Some(sum)
		} else {
			None
		}
	})
}

fn part1(calories: impl Iterator<Item = u64>) -> u64 {
	calories.max().unwrap_or_default()
}

fn part2(calories: impl Iterator<Item = u64>) -> u64 {
	let mut calories = calories.collect::<Vec<_>>();
	calories.sort_unstable_by(|a, b| b.cmp(a));
	calories.into_iter().take(3).sum()
}

fn main() {
  println!("--- day1 ---");
	println!("part 1 => {}", part1(calories(INPUT)));
	println!("part 2 => {}", part2(calories(INPUT)));
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(calories(INPUT)), 24000);
	}
	
	#[test]
	fn test_part2() {
		assert_eq!(part2(calories(INPUT)), 45000);
	}
}