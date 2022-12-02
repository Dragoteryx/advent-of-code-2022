const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Shape {
	Rock,
	Paper,
	Scissors
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MatchOutcome {
	Won,
	Draw,
	Lost
}

fn outcome(played: Shape, opponent: Shape) -> MatchOutcome {
	match (played, opponent) {
		(Shape::Rock, Shape::Paper) => MatchOutcome::Lost,
		(Shape::Rock, Shape::Scissors) => MatchOutcome::Won,
		(Shape::Paper, Shape::Rock) => MatchOutcome::Won,
		(Shape::Paper, Shape::Scissors) => MatchOutcome::Lost,
		(Shape::Scissors, Shape::Rock) => MatchOutcome::Lost,
		(Shape::Scissors, Shape::Paper) => MatchOutcome::Won,
		_ => MatchOutcome::Draw
	}
}

fn calc_score(played: Shape, opponent: Shape) -> u64 {
	let mut score = match played {
		Shape::Rock => 1,
		Shape::Paper => 2,
		Shape::Scissors => 3
	};

	match outcome(played, opponent) {
		MatchOutcome::Won => score += 6,
		MatchOutcome::Draw => score += 3,
		MatchOutcome::Lost => score += 0
	}

	score
}

fn part1(input: &str) -> u64 {
	input.lines().map(|s| {
		let mut chars = s.chars();
		
		let opponent = match chars.next() {
			Some('A') => Shape::Rock,
			Some('B') => Shape::Paper,
			Some('C') => Shape::Scissors,
			_ => panic!("invalid input")
		};

		chars.next();

		let played = match chars.next() {
			Some('X') => Shape::Rock,
			Some('Y') => Shape::Paper,
			Some('Z') => Shape::Scissors,
			_ => panic!("invalid input")
		};

		calc_score(played, opponent)
	}).sum()
}

fn part2(input: &str) -> u64 {
	input.lines().map(|s| {
		let mut chars = s.chars();

		let opponent = match chars.next() {
			Some('A') => Shape::Rock,
			Some('B') => Shape::Paper,
			Some('C') => Shape::Scissors,
			_ => panic!("invalid input")
		};

		chars.next();

		let desired_outcome = match chars.next() {
			Some('X') => MatchOutcome::Lost,
			Some('Y') => MatchOutcome::Draw,
			Some('Z') => MatchOutcome::Won,
			_ => panic!("invalid input")
		};

		let should_play = match (opponent, desired_outcome) {
			(Shape::Rock, MatchOutcome::Won) => Shape::Paper,
			(Shape::Rock, MatchOutcome::Lost) => Shape::Scissors,
			(Shape::Paper, MatchOutcome::Won) => Shape::Scissors,
			(Shape::Paper, MatchOutcome::Lost) => Shape::Rock,
			(Shape::Scissors, MatchOutcome::Won) => Shape::Rock,
			(Shape::Scissors, MatchOutcome::Lost) => Shape::Paper,
			(_, MatchOutcome::Draw) => opponent,
		};

		calc_score(should_play, opponent)
	}).sum()
}

fn main() {
	println!("--- day 2 ---");
	println!("part 1 => {}", part1(INPUT));
	println!("part 2 => {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 15);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 12);
	}
}