use rand::{seq::SliceRandom, thread_rng};

use super::{Cell, World};

pub const SIZE: isize = 16;
const U_SIZE: usize = SIZE as usize;

type Move = (isize, isize, isize, isize);

#[derive(Debug)]
pub struct Chunk {
  cells: Vec<Cell>,
	pub changes: Vec<Move>,
  pub x: isize,
  pub y: isize,
}

impl Chunk {
  pub fn new(x: isize, y: isize) -> Self {
    Self { 
      x, 
      y, 
      changes: Vec::new(),
      cells: vec![Cell::Air; (U_SIZE * U_SIZE).try_into().unwrap()], 
    }
  }

  pub fn in_bounds(&self, x: isize, y: isize) -> bool {
    x >= self.x && x < self.x + SIZE &&
      y >= self.y && y < self.y + SIZE
  }

  pub fn index(&self, x: isize, y: isize) -> usize {
    (x - self.x + (y - self.y) * SIZE) as usize
  }

  pub fn empty(&self, x: isize, y: isize) -> bool {
    match self.cells.get(self.index(x, y)) {
      Some(Cell::Air) => true,
      _ => false
    }
  }

	pub fn set_cell(&mut self, x: isize, y: isize, cell: Cell) {
    let index = self.index(x, y);
    self.cells[index] = cell;
	}

	pub fn get_cell(&self, x: isize, y: isize) -> Option<Cell> {
		self.cells.get(self.index(x, y)).copied()
	}


  pub fn move_cell(&mut self, from_x: isize, from_y: isize, to_x: isize, to_y: isize) {
    self.changes.push((to_x, to_y, from_x, from_y)) 
  }

  pub fn commit_changes(&mut self, world: &mut World) {
    commit_chunk_changes(world, self);
  }
}

pub fn commit_chunk_changes(world: &mut World, chunk: &mut Chunk) {
  chunk.changes.shuffle(&mut thread_rng());

  while let Some((to_x, to_y, from_x, from_y)) = chunk.changes.pop() {

    let from_cell = if chunk.in_bounds(from_x, from_y) { 
      chunk.get_cell(from_x, from_y) 
    } else {
      world.get_cell(from_x, from_y)
    };

    match (from_cell, chunk.get_cell(to_x, to_y)) {
      (Some(from), Some(to)) if to.density() < from.density() => {
        chunk.set_cell(to_x, to_y, from);

        if chunk.in_bounds(from_x, from_y) { 
          chunk.set_cell(from_x, from_y, to);
        } else {
          world.set_cell(from_x, from_y, to);
        };

      },
      _ => { }
    }
  }

}