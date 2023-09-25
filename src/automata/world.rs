use std::{collections::HashMap, hash::Hash, cell::RefCell, borrow::BorrowMut, rc::Rc, ops::{Deref, DerefMut}};

use rand::{seq::SliceRandom, rngs::ThreadRng, thread_rng};

use super::{Cell, chunk::{Chunk, self, commit_chunk_changes}};

type Move = (isize, isize, isize, isize);

type StoredChunk = Rc<RefCell<Chunk>>;

#[derive(Debug)]
pub struct World {
	chunks_map: HashMap<(isize, isize), StoredChunk>,
	chunks: Vec<StoredChunk>
}

impl World {
	pub fn new(width: isize, height: isize) -> Self {
		Self { chunks_map: HashMap::new(), chunks: Vec::new() }
	}

	fn lookup_chunk(&self, x: isize, y: isize) -> Option<&StoredChunk> {
		self.chunks_map.get(&( x / chunk::SIZE, y / chunk::SIZE ))
	}

	pub fn get_chunk(&self, index: usize) -> Option<&StoredChunk> {
		self.chunks.get(index)
	}

	pub fn chunk_length(&self) -> usize {
		self.chunks.len()
	}

	pub fn set_cell(&mut self, x: isize, y: isize, cell: Cell) {
		if let Some(chunk) = self.lookup_chunk(x, y) {
			let chunk = chunk.deref();
			chunk.borrow_mut().set_cell(x, y, cell);
		} else {
			let mut chunk_x = (x / chunk::SIZE) * chunk::SIZE;
			let mut chunk_y = (y / chunk::SIZE) * chunk::SIZE;
			if x < 0 { chunk_x -= 1; }
			if y < 0 { chunk_y -= 1; }

			let mut chunk = Chunk::new(chunk_x, chunk_y);
			chunk.set_cell(x, y, cell);
			let chunk = Rc::new(RefCell::new(chunk));
			self.chunks_map.insert(( x / chunk::SIZE, y / chunk::SIZE ), chunk.clone());
			self.chunks.push(chunk)
		}
	}

	pub fn get_cell(&self, x: isize, y: isize) -> Option<Cell> {
		self.lookup_chunk(x, y)?.borrow().get_cell(x, y)
	}

	pub fn move_cell(&mut self, from_x: isize, from_y: isize, to_x: isize, to_y: isize) {
		if let Some(chunk) = self.lookup_chunk(to_x, to_y) {
			let chunk = chunk.deref();
			chunk.borrow_mut().move_cell(from_x, from_y, to_x, to_y);
		}
	}

	pub fn commit_changes(&mut self) {
		for i in 0..self.chunks.len() {
			let chunk = self.chunks[i].clone();
			let chunk = chunk.deref();
			chunk.borrow_mut().commit_changes(self);
		}
	}

}