use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;
use std::fmt::{self, Debug};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Heightmap {
	squares: Vec<Vec<u8>>,
	start: (usize, usize),
	end: (usize, usize)
}

impl Heightmap {
	pub fn new(
		start: (usize, usize),
		end: (usize, usize),
		squares: Vec<Vec<u8>>
	) -> Self {
		Self {
			squares,
			start,
			end
		}
	}

	pub fn start(&self) -> Square {
		self.square(self.start.0, self.start.1)
			.expect("invalid start position")
	}

	pub fn end(&self) -> Square {
		self.square(self.end.0, self.end.1)
			.expect("invalid end position")
	}

	pub fn square(&self, x: usize, y: usize) -> Option<Square> {
		if let Some(line) = self.squares.get(y) {
			if let Some(&elevation) = line.get(x) {
				return Some(Square {
					heightmap: self,
					elevation, x, y
				});
			}
		}

		None
	}

	pub fn squares(&self) -> impl Iterator<Item = Square> {
		self.squares.iter().enumerate().flat_map(move |(y, line)| {
			line.iter().enumerate().map(move |(x, &elevation)| {
				Square {
					heightmap: self,
					elevation, x, y
				}
			})
		})
	}
}

impl Debug for Heightmap {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Heightmap ")?;
		f.debug_list()
			.entries(self.squares())
			.finish()
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square<'h> {
	heightmap: &'h Heightmap,
	elevation: u8,
	x: usize,
	y: usize
}

impl<'h> Square<'h> {
	pub fn elevation(self) -> u8 {
		self.elevation
	}

	pub fn neighbors(self) -> impl Iterator<Item = Square<'h>> {
		let neighbors = [
			self.heightmap.square(self.x, self.y - 1),
			self.heightmap.square(self.x + 1, self.y),
			self.heightmap.square(self.x, self.y + 1),
			self.heightmap.square(self.x - 1, self.y)
		];
	
		neighbors.into_iter().flatten()
	}

	pub fn reachable_neighbors(self) -> impl Iterator<Item = Square<'h>> {
		self.neighbors().filter(move |neighbor| {
			self.elevation.abs_diff(neighbor.elevation) < 2 || neighbor.elevation < self.elevation
		})
	}
	
	pub fn shortest_path(self, goal: Self) -> Option<VecDeque<Self>> {
		let mut squares = VecDeque::from([self]);
		let mut came_from = HashMap::new();
		while let Some(current) = squares.pop_front() {
			if goal == current {
				let mut current = current;
				let mut path = VecDeque::from([current]);
				while let Some(from) = came_from.remove(&current) {
					if from == self { break; }
					path.push_front(from);
					current = from;
				}
				return Some(path);
			} else {
				for neighbor in current.reachable_neighbors() {
					if let Entry::Vacant(entry) = came_from.entry(neighbor) {
						squares.push_back(neighbor);
						entry.insert(current);
					}
				}
			}
		}

		None
	}
}

impl Debug for Square<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Square")
			.field("x", &self.x)
			.field("y", &self.y)
			.field("elevation", &self.elevation)
			.finish()
	}
}