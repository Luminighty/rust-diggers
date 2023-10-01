use std::{rc::Rc, cell::RefCell};

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

		self.cells[(x + y * WIDTH) as usize] = Some(cell);
	}

	pub fn move_cell(&mut self, from_x: isize, from_y: isize, to_x: isize, to_y: isize) {
		self.cells.swap((from_x + from_y * WIDTH) as usize, (to_x + to_y * WIDTH) as usize);
	}

	pub fn get_cell(&self, x: isize, y: isize) -> Option<&StoredCell> {
		if let Some(cell) = &self.cells.get((x + y * WIDTH) as usize) {
			cell.as_ref()
		} else {
			None
		}
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
