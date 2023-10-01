use rand::{thread_rng, Rng};

use super::{CellData, cell::{CellBehaviour, Density, Color}};

#[derive(Clone, Copy, Debug)]
pub enum Cell {
	Air,
	Sand,
	Rock,
	Water,
	Slime,
	Smoke,
	Dirt,
}

impl Cell {
	pub fn build(self) -> CellData {
		CellData::new(self, self.color())
	}

	pub fn color(&self) -> Color {
		let mut rng = thread_rng();
		match self {
			Cell::Air => color(0, 0, 0),
			Cell::Sand => color(rng.gen_range(0xe8..=0xff), rng.gen_range(0xa9..=0xbf), rng.gen_range(0x00..=0x10)),
			Cell::Rock => color(rng.gen_range(0x28..0x30), rng.gen_range(0x38..0x40), rng.gen_range(0x48..0x50)),
			Cell::Water => color(0x00, rng.gen_range(0x80..0x90), rng.gen_range(0xf0..=0xff)),
			Cell::Slime => color(rng.gen_range(0x66..=0x77), 0x20, rng.gen_range(0xaa..=0xbb)),
			Cell::Smoke => color(0x30, 0x30, 0x30),
			Cell::Dirt => color(rng.gen_range(0x50..0x55), rng.gen_range(0x40..0x49), rng.gen_range(0x30..0x45)),
		}
	}

	pub fn behaviour(&self) -> CellBehaviour {
		match self {
			Cell::Air => CellBehaviour::Static,
			Cell::Sand => CellBehaviour::Powder,
			Cell::Rock => CellBehaviour::Static,
			Cell::Water => CellBehaviour::Liquid,
			Cell::Slime => CellBehaviour::Liquid,
			Cell::Smoke => CellBehaviour::Gas,
			Cell::Dirt => CellBehaviour::Powder,
		}
	}

	pub fn density(&self) -> Density {
		match self {
			Cell::Air => 0,
			Cell::Sand => 100,
			Cell::Rock => 1000,
			Cell::Water => 10,
			Cell::Slime => 50,
			Cell::Smoke => 1,
			Cell::Dirt => 90,
		}
	}

}

fn color(r: u8, g: u8, b: u8) -> [u8; 4] {
	[r, g, b, 0xff]
}
