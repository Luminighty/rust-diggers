use super::CellData;

#[derive(Debug)]
pub struct World { }

impl World {
	pub fn new() -> Self { 
		Self {}
	}

	pub fn get_cell(&self, x: isize, y: isize) -> Option<CellData> {
		None
	}
}