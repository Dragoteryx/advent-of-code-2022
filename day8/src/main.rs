mod forest;
use forest::*;

fn forest(input: &str) -> Forest {
	Forest::new(input.lines().map(|line| {
		line.chars()
			.map(|c| c.to_string())
			.map(|c| c.parse().expect("invalid input"))
			.collect()
	}).collect())
}

fn part1(forest: &Forest) -> usize {
	forest.trees().filter(|tree| tree.is_visible()).count()
}

fn part2(forest: &Forest) -> usize {
	forest.trees().map(|tree| tree.scenic_score()).max().unwrap()
}

fn main() {
	let forest = forest(include_str!("real_input.txt"));
	println!("part 1 => {}", part1(&forest));
	println!("part 2 => {}", part2(&forest));
}

#[test]
fn test() {
	let forest = forest(include_str!("test_input.txt"));
	assert_eq!(part1(&forest), 21);
	assert_eq!(part2(&forest), 8);
}