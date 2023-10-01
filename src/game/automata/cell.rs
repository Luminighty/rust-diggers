use rand::{thread_rng, seq::SliceRandom, Rng};

use super::Cell;

pub type Color = [u8; 4];
pub type Density = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellBehaviour {
	Static,
	Powder,
	Liquid,
	Gas,
}

impl CellBehaviour {
	pub fn steps(&self) -> std::slice::Iter<'_, (isize, isize)> {
		let mut rng = thread_rng();
		
		match self {
			Self::Powder => {
				if rng.gen_bool(0.5) {
					[(0, 1), (-1, 1), (1, 1)].iter()
				} else {
					[(0, 1), (1, 1), (-1, 1)].iter()
				}
			},
			Self::Liquid => {
				if rng.gen_bool(0.5) {
					[(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)].iter()
				} else {
					[(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)].iter()
				}
			}
			Self::Gas => {
				if rng.gen_bool(0.5) {
					[(0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0)].iter()
				} else {
					[(0, -1), (1, -1), (-1, -1), (1, 0), (-1, 0)].iter()
				}
			},
			Self::Static => [].iter(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct CellData {
	pub is_sleeping: bool,
	pub color: Color,
	pub cell: Cell,
}

impl CellData {
	pub fn new(cell: Cell, color: Color) -> Self {
		Self { 
			is_sleeping: cell.behaviour() == CellBehaviour::Static,
			color, cell
		}
	}

	pub fn behaviour(&self) -> CellBehaviour {
		self.cell.behaviour()
	}
	
	pub fn density(&self) -> Density {
		self.cell.density()
	}
}
