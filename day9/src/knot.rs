use std::cmp::Ordering;
use super::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Knot {
	pub x: i32,
	pub y: i32
}

impl Knot {
	pub fn is_touching(&self, other: Self) -> bool {
		self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
	}

	pub fn goto(&mut self, mov: Move) {
		match mov {
			Move::Up => self.y += 1,
			Move::Down => self.y -= 1,
			Move::Left => self.x -= 1,
			Move::Right => self.x += 1
		}
	}

	pub fn join(&mut self, other: Self) {
		while !self.is_touching(other) {
			match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
				(Ordering::Greater, Ordering::Equal) => self.goto(Move::Left),
				(Ordering::Less, Ordering::Equal) => self.goto(Move::Right),
				(Ordering::Equal, Ordering::Greater) => self.goto(Move::Down),
				(Ordering::Equal, Ordering::Less) => self.goto(Move::Up),
				(Ordering::Greater, Ordering::Greater) => {
					self.goto(Move::Left);
					self.goto(Move::Down);
				}
				(Ordering::Greater, Ordering::Less) => {
					self.goto(Move::Left);
					self.goto(Move::Up);
				}
				(Ordering::Less, Ordering::Greater) => {
					self.goto(Move::Right);
					self.goto(Move::Down);
				}
				(Ordering::Less, Ordering::Less) => {
					self.goto(Move::Right);
					self.goto(Move::Up);
				}
				_ => unreachable!()
			}
		}
	}
}