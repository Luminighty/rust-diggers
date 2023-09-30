use super::{CellData, cell::{CellBehaviour, Density}};

pub enum Cell {
	Air
}

impl Cell {
	pub fn build(&self) -> CellData {
		match self {
			Self::Air => CellData::new(CellBehaviour::Static, [0, 0, 0, 0xff], Density::MAX),
		}
	}
}