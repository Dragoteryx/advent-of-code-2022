use std::cell::{Cell, RefCell};
use std::fmt::{self, Debug};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
	Mult(u128),
	Add(u128),
	Square,
}

/// Return to monke.
pub struct Monke {
	items: RefCell<Vec<u128>>,
	inspected: Cell<u128>,
	operation: Operation,
	test: u128,
	if_true: usize,
	if_false: usize
}

impl Debug for Monke {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Monke")
			.field("items", &self.items)
			.field("if_true", &self.if_true)
			.field("if_false", &self.if_false)
			.finish()
	}
}

impl Monke {
	pub fn new(
		items: Vec<u128>,
		operation: Operation,
		test: u128,
		if_true: usize,
		if_false: usize
	) -> Self {
		Self {
			items: RefCell::new(items), 
			inspected: Cell::new(0),
			operation, test,
			if_true, if_false
		}
	}

	pub fn test(&self) -> u128 {
		self.test
	}

	pub fn inspected(&self) -> u128 {
		self.inspected.get()
	}

	pub fn play_turn(&self, monkeys: &[Self], mut manage_worry: impl FnMut(u128) -> u128) {
		for old_worry_level in self.items.borrow_mut().drain(..) {
			self.inspected.set(self.inspected.get() + 1);

			let new_worry_level = manage_worry(match self.operation {
				Operation::Square => old_worry_level.pow(2),
				Operation::Mult(n) => old_worry_level * n,
				Operation::Add(n) => old_worry_level + n
			});

			let monke = if new_worry_level % self.test == 0 {
				&monkeys[self.if_true]
			} else {
				&monkeys[self.if_false]
			};

			monke.items.borrow_mut().push(new_worry_level);
		}
	}
}