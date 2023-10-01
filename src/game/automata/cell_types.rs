use rand::{thread_rng, Rng};

use super::{CellData, cell::CellBehaviour};

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
	pub fn build(&self) -> CellData {
		let mut rng = thread_rng();
		match self {
			Self::Air => CellData { 
				behaviour: CellBehaviour::Static, 
				color: color(0, 0, 0), 
				density: 0
			},
			Self::Sand => CellData { 
				behaviour: CellBehaviour::Powder, 
				color: color(rng.gen_range(0xe8..=0xff), rng.gen_range(0xa9..=0xbf), rng.gen_range(0x00..=0x10)),
				density: 100
			},
			Self::Rock => CellData { 
				behaviour: CellBehaviour::Static, 
				color: color(rng.gen_range(0x28..0x30), rng.gen_range(0x38..0x40), rng.gen_range(0x48..0x50)),
				density: 1000
			},
			Self::Water => CellData { 
				behaviour: CellBehaviour::Liquid, 
				color: color(0x00, rng.gen_range(0x80..0x90), rng.gen_range(0xf0..=0xff)),
				density: 10
			},
			Self::Slime => CellData { 
				behaviour: CellBehaviour::Liquid, 
				color: color(rng.gen_range(0x66..=0x77), 0x20, rng.gen_range(0xaa..=0xbb)),
				density: 50
			},
			Self::Smoke => CellData { 
				behaviour: CellBehaviour::Gas, 
				color: color(0x30, 0x30, 0x30),
				density: 1
			},
			Self::Dirt => CellData { 
				behaviour: CellBehaviour::Powder, 
				color: color(rng.gen_range(0x50..0x55), rng.gen_range(0x40..0x49), rng.gen_range(0x30..0x45)),
				density: 90
			},
		}
	}
}

fn color(r: u8, g: u8, b: u8) -> [u8; 4] {
	[r, g, b, 0xff]
}
