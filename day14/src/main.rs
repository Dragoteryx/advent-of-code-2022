mod parse;

mod cave;
use cave::*;

fn cave(input: &str) -> Cave {
	let paths = parse::input(input);
	let mut cave = Cave::default();
	for path in paths {
		cave.add_rocks(path);
	}
	
	cave
}

fn part1(input: &str) -> usize {
	let mut cave = cave(input);
	let mut i = 0;
	'outer: loop {
		let (mut x, mut y) = (500, 0);
		cave.set_tile(x, y, Tile::Sand);
		loop {
			match cave.update_sand(x, y) {
				Some((_, y2)) if y2 == cave.lowest_y() => break 'outer i,
				None => break,
				Some((x2, y2)) => {
					x = x2;
					y = y2;
				}
			}
		}

		i += 1;
	}
}

fn part2(input: &str) -> usize {
	let mut cave = cave(input);
	let mut i = 0;
	'outer: loop {
		i += 1;

		let (mut x, mut y) = (500, 0);
		cave.set_tile(x, y, Tile::Sand);
		loop {
			match cave.update_sand(x, y) {
				None if (500, 0) == (x, y) => break 'outer i,
				None => break,
				Some((x2, y2)) => {
					x = x2;
					y = y2;
				}
			}
		}
	}
}

const INPUT: &str = include_str!("real_input.txt");

fn main() {
	println!("--- day 14 ---");
	println!("part 1 => {}", part1(INPUT));
	println!("part 2 => {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 24);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 93);
	}
}