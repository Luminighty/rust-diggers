use rand::{thread_rng, Rng};

use super::World;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
	Air,
	Sand,
	Water,
	Rock,
	Smoke,
	Slime,
	Oil,
	Dirt,
}

impl Cell {

	pub fn step(&self, x: isize, y: isize, world: &mut World) -> Option<(isize, isize)> {
		for (dx, dy) in self.steps() {
			let (target_x, target_y) = (x + dx, y + dy);
			if let Some(other) = world.get_cell(target_x, target_y) {
				if other.density() < self.density() {
					return Some((target_x, target_y));
				}
			} else {
				return Some((target_x, target_y));
			}
		}
		None
	}

	fn steps(&self) -> std::slice::Iter<'_, (isize, isize)> {
		match self {
			Cell::Sand => [(0, 1), (-1, 1), (1, 1)].iter(),
			Cell::Water => [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)].iter(),
			Cell::Slime => [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)].iter(),
			Cell::Oil => [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)].iter(),
			Cell::Smoke => [(0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0)].iter(),
			Cell::Dirt => [(0, 1), (-1, 1), (1, 1)].iter(),
			Cell::Air => [].iter(),
			Cell::Rock => [].iter()
		}
	}

	pub fn density(&self) -> usize {
		match self {
			Cell::Air => 0,
			Cell::Rock => 1000,
			Cell::Sand => 200,
			Cell::Water => 100,
			Cell::Smoke => 30,
			Cell::Slime => 150,
			Cell::Oil => 80,
			Cell::Dirt => 200,
		}
	}

}
