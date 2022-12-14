use std::collections::HashMap;
use std::fmt::{self, Display};
use std::cmp::Ordering;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
	#[default]
	Air,
	Rock,
	Sand
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Cave {
	tiles: HashMap<(usize, usize), Tile>
}

impl Cave {
	pub fn tile(&self, x: usize, y: usize) -> Tile {
		if y != self.lowest_y() + 2 {
			self.tiles.get(&(x, y)).copied().unwrap_or_default()
		} else {
			Tile::Rock
		}
	}

	pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
		if tile != Tile::Air {
			self.tiles.insert((x, y), tile);
		} else {
			self.tiles.remove(&(x, y));
		}
	}

	pub fn leftest_x(&self) -> usize {
		self.tiles.iter()
			.map(|(&pos, &tile)| (pos, tile))
			.filter(|&(_, tile)| tile == Tile::Rock)
			.map(|((x, _), _)| x)
			.min().unwrap_or(500)
	}

	pub fn rightest_x(&self) -> usize {
		self.tiles.iter()
			.map(|(&pos, &tile)| (pos, tile))
			.filter(|&(_, tile)| tile == Tile::Rock)
			.map(|((x, _), _)| x)
			.max().unwrap_or(500)
	}

	pub fn lowest_y(&self) -> usize {
		self.tiles.iter()
			.map(|(&pos, &tile)| (pos, tile))
			.filter(|&(_, tile)| tile == Tile::Rock)
			.map(|((_, y), _)| y)
			.max().unwrap_or(0)
	}

	pub fn add_rocks(&mut self, rocks: impl IntoIterator<Item = (usize, usize)>) {
		let mut iter = rocks.into_iter().peekable();
		while let Some((x1, y1)) = iter.next() {
			match iter.peek().copied() {
				None => self.set_tile(x1, y1, Tile::Rock),
				Some((x2, y2)) => match (x1.cmp(&x2), y1.cmp(&y2)) {
					(Ordering::Equal, Ordering::Equal) => self.set_tile(x1, y1, Tile::Rock),
					(Ordering::Less, Ordering::Equal) => for x in x1..x2 {
						self.set_tile(x, y1, Tile::Rock);
					}
					(Ordering::Greater, Ordering::Equal) => for x in (x2 + 1)..=x1 {
						self.set_tile(x, y1, Tile::Rock);
					}
					(Ordering::Equal, Ordering::Less) => for y in y1..y2 {
						self.set_tile(x1, y, Tile::Rock);
					}
					(Ordering::Equal, Ordering::Greater) => for y in (y2 + 1)..=y1 {
						self.set_tile(x1, y, Tile::Rock);
					}
					_ => panic!("invalid rock path")
				}
			}
		}
	}

	pub fn update_sand(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
		if self.tile(x, y) != Tile::Sand {
			panic!("not a sand tile");
		} else if self.tile(x, y + 1) == Tile::Air {
			self.set_tile(x, y + 1, Tile::Sand);
			self.set_tile(x, y, Tile::Air);
			Some((x, y + 1))
		} else if self.tile(x - 1, y + 1) == Tile::Air {
			self.set_tile(x - 1, y + 1, Tile::Sand);
			self.set_tile(x, y, Tile::Air);
			Some((x - 1, y + 1))
		} else if self.tile(x + 1, y + 1) == Tile::Air {
			self.set_tile(x + 1, y + 1, Tile::Sand);
			self.set_tile(x, y, Tile::Air);
			Some((x + 1, y + 1))
		} else {
			None
		}
	}

	#[allow(deprecated)]
	pub fn print_and_sleep(&self) {
		println!("\x1B[2J\x1B[1;1H{self}");
		std::thread::sleep_ms(10);
	}
}

impl Display for Cave {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let lowest_y = self.lowest_y() + 2;

		for y in 0..=lowest_y {
			for x in (self.leftest_x() - lowest_y)..=(self.rightest_x() + lowest_y) {
				if x == 500 && y == 0 {
					write!(f, "+")?;
				} else {
					match self.tile(x, y) {
						Tile::Rock => write!(f, "█")?,
						Tile::Sand => write!(f, "#")?,
						Tile::Air => write!(f, " ")?
					}
				}
			}
			if y < lowest_y {
				writeln!(f)?;
			}
		}

		Ok(())
	}
}