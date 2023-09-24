use super::Cell;

type Move = (isize, isize, isize, isize);

#[derive(Debug, Clone)]
pub struct World {
	pub height: isize,
	pub width: isize,
	pub cells: Vec<Cell>,
	changes: Vec<Move>
}

impl World {
	pub fn new(width: isize, height: isize) -> Self {
		let cells = Vec::from_iter((0..height*width).map(|_| Cell::Air).collect::<Vec<Cell>>());
		Self { cells, width, height, changes: Vec::new() }
	}

  fn index(&self, x: isize, y: isize) -> Option<usize> {
		let x: usize = x.try_into().ok()?;
		let y: usize = y.try_into().ok()?;
    Some(self.width as usize * y + x)
  }

	pub fn set_cell(&mut self, x: isize, y: isize, cell: Cell){
		if x < 0 || x >= self.width {
			return;
		}
		if y < 0 || y >= self.height {
			return;
		}
    if let Some(index) = self.index(x, y) {
      self.cells[index] = cell;
    }
	}

	pub fn get_cell(&self, x: isize, y: isize) -> Option<&Cell> {
		self.cells.get(self.index(x, y)?)
	}

	pub fn is_empty(&self, x: isize, y: isize) -> bool {
		match self.get_cell(x, y) {
			Some(&Cell::Air) => true,
			_ => false,
		}
	}

	pub fn move_cell(&mut self, from_x: isize, from_y: isize, to_x: isize, to_y: isize) {
		self.changes.push((to_x, to_y, from_x, from_y)) // Want to sort on target position
	}

	pub fn commit_changes(&mut self) {
		self.changes.sort_unstable();
		let mut last_target = None;
		for (to_x, to_y, from_x, from_y) in self.changes.clone() {
			if let Some((last_x, last_y)) = last_target {
				if last_x == to_x && last_y == to_y {
					continue;
				}
			}
			if let Some(cell) = self.get_cell(from_x, from_y) {
				self.set_cell(to_x, to_y, *cell);
				self.set_cell(from_x, from_y, Cell::Air);
			}
			
			last_target = Some((to_x, to_y))
		}
		self.changes.clear()
	}

	pub fn print(&self) {
		let width = self.width as isize;
		let height = self.height as isize;
		for y in 0..height {
			for x in 0..width {
				if let Some(cell) = self.get_cell(x, y) {
					print!("{}", match cell {
						Cell::Air => " ",
						Cell::Rock => "X",
						Cell::Sand => "O",
						Cell::Water => "~",
					})
				}
			}
			println!()
		}
		println!()
	}
}