pub mod world;
pub mod cell;
mod chunk;

use std::borrow::BorrowMut;

pub use world::World;
pub use cell::Cell;

fn simulate_cell(x: isize, y: isize, world: &mut World) {
	if let Some(cell) = world.get_cell(x, y) {
		if let Some((target_x, target_y)) = cell.step(x, y, world) {
			world.move_cell(x, y, target_x, target_y);
		}
	}
}

pub fn step(world: &mut World) {
	for i in 0..world.chunk_length() {
		let chunk = world.get_chunk(i).unwrap();
		let chunk_x = chunk.as_ref().borrow().x;
		let chunk_y = chunk.as_ref().borrow().y;
		drop(chunk);
		for x in 0..chunk::SIZE {
			for y in 0..chunk::SIZE {
				simulate_cell(chunk_x + x, chunk_y + y, world)
			}
		}
	}
	world.commit_changes();
}
