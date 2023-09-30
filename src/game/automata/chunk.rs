use rand::{seq::SliceRandom, thread_rng};

use super::{CellData, World, cell_types::Cell};

pub const SIZE: isize = 64;
const U_SIZE: usize = SIZE as usize;

type Move = (isize, isize, isize, isize);

#[derive(Debug)]
pub struct Chunk {
  cells: [CellData; U_SIZE * U_SIZE],
  pub x: isize,
  pub y: isize,
}

impl Chunk {
  pub fn new(x: isize, y: isize) -> Self {
    Self { x, y, 
      cells: [Cell::Air.build(); U_SIZE * U_SIZE], 
    }
  }

	pub fn set_cell(&mut self, x: isize, y: isize, cell: CellData) {
    if let Some(index) = self.index(x, y) {
      self.cells[index] = cell;
    } else {
      println!("[WARN] Chunk index is out of bounds: Chunk: ({}, {}), Setting: ({}, {})", self.x, self.y, x, y);
    }
	}

	pub fn get_cell(&self, x: isize, y: isize) -> Option<CellData> {
		self.cells.get(self.index(x, y)?).copied()
	}

  fn index(&self, x: isize, y: isize) -> Option<usize> {
    let x = x + self.x;
    let y = y + self.y;
    if x < 0 || x >= SIZE {
      return None;
    } 
    if y < 0 || y >= SIZE {
      return None;
    }
    return Some((y * SIZE + x) as usize);
  }
}
