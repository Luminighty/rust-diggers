use std::{rc::Rc, cell::RefCell, borrow::BorrowMut};

use crate::{WIDTH, HEIGHT};

use super::cell::CellData;

pub type StoredCell = Rc<RefCell<CellData>>;


#[derive(Debug)]
pub struct World {
	pub cells: Vec<Option<StoredCell>>,
	pub actions: Actions,
}

impl World {
	pub fn new() -> Self { 
		Self {
			cells: vec![None; (WIDTH * HEIGHT) as usize],
			actions: Actions::new(),
		}
	}

	pub fn set_cell(&mut self, x: isize, y: isize, cell: CellData) {
		let cell = Rc::new(RefCell::new(cell));
		let i = World::index(x, y);
		self.cells[i] = Some(cell);
	}

	pub fn move_cell(&mut self, from_x: isize, from_y: isize, to_x: isize, to_y: isize) {
		let from = World::index(from_x, from_y);
		let to = World::index(to_x, to_y);
		self.cells.swap(from, to);
	}

	pub fn get_cell(&self, x: isize, y: isize) -> Option<&StoredCell> {
		let index = World::index(x, y);
		if let Some(cell) = &self.cells.get(index) {
			cell.as_ref()
		} else {
			None
		}
	}

	pub fn set_sleep(&mut self, x: isize, y: isize, is_sleeping: bool) {
		let index = World::index(x, y);
		if let Some(cell) = &self.cells.get(index) {
			if let Some(cell) = cell {
				let cell = Rc::clone(cell);
				let mut cell = (*cell).borrow_mut();
				cell.is_sleeping = is_sleeping;
			}
		}
	}

	pub fn index(x: isize, y: isize) -> usize {
		(x + y * WIDTH) as usize
	}
}

#[derive(Debug)]
pub struct Actions {
	pub actions: Vec<((isize, isize), (isize, isize))>
}

impl Actions {
	pub fn new() -> Self {
		Self { actions: Vec::new() }
	}

	pub fn move_cell(&mut self, from_x: isize, from_y: isize, to_x: isize, to_y: isize) {
		self.actions.push(((to_x, to_y), (from_x, from_y)));
	}

	pub fn clear(&mut self) {
		self.actions.clear();
	}
}
