mod heightmap;
use heightmap::*;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;

fn heightmap(input: &str) -> Heightmap {
	let (mut start, mut end) = ((0, 0), (0, 0));
	let squares = input.lines()
		.enumerate()
		.map(|(y, line)| {
			line.chars()
				.enumerate()
				.map(|(x, c)| match c {
					'a'..='z' => c as u8 - 97,
					'S' => {
						start = (x, y);
						0
					}
					'E' => {
						end = (x, y);
						25
					}
					_ => unreachable!()
				}).collect()
		}).collect();

	Heightmap::new(start, end, squares)
}

fn part1(heightmap: &Heightmap) -> usize {
	heightmap.start().shortest_path(heightmap.end()).unwrap().len()
}

/// Rayon ❤️
/// This most definitely isn't the most efficient way to do this but I'm lazy.
fn part2(heightmap: &Heightmap) -> usize {
	heightmap.squares()
		.filter(|sq| sq.elevation() == 0)
		.collect::<Vec<_>>()
		.into_par_iter()
		.flat_map(|sq| sq.shortest_path(heightmap.end()))
		.map(|path| path.len())
		.min().unwrap()
}

fn main() {
	let heightmap = heightmap(include_str!("real_input.txt"));

	println!("--- day 12 ---");
	println!("part 1 => {}", part1(&heightmap));
	println!("part 2 => {}", part2(&heightmap));
}

#[test]
fn test() {
	let heightmap = heightmap(include_str!("test_input.txt"));
	assert_eq!(part1(&heightmap), 31);
	assert_eq!(part2(&heightmap), 29);
}