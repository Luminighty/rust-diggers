use rand::{thread_rng, Rng};

use super::{CellData, cell::{CellBehaviour, Density}};

pub enum Cell {
	Air,
	Sand,
	Rock,
	Water,
}

impl Cell {
	pub fn build(&self) -> CellData {
		let mut rng = thread_rng();
		
		match self {
			Self::Air => 
				CellData::new(CellBehaviour::Static, [0, 0, 0, 0xff], 0),
			Self::Sand =>
				CellData::new(CellBehaviour::Powder, [rng.gen_range(0xe8..=0xff), rng.gen_range(0xa9..=0xbf), rng.gen_range(0x00..=0x10), 0xff], 100),
			Self::Rock =>
				CellData::new(CellBehaviour::Static, [rng.gen_range(0x28..0x30), rng.gen_range(0x38..0x40), rng.gen_range(0x48..0x50), 0xff], 1000),
			Self::Water =>
				CellData::new(CellBehaviour::Liquid, [0x00, rng.gen_range(0x80..0x90), rng.gen_range(0xf0..=0xff), 0xff], 10),

		}
	}
}