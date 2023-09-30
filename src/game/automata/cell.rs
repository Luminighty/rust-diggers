pub type Color = [u8; 4];
pub type Density = u16;

#[derive(Debug, Clone, Copy)]
pub enum CellBehaviour {
	Static,
	Powder,
	Liquid,
	Gas,
}

impl CellBehaviour {
	pub fn steps(&self) -> std::slice::Iter<'_, (isize, isize)> {
		match self {
			Self::Powder => [(0, 1), (-1, 1), (1, 1)].iter(),
			Self::Liquid => [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)].iter(),
			Self::Gas => [(0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0)].iter(),
			Self::Static => [].iter(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct CellData {
	pub color: Color,
	pub behaviour: CellBehaviour,
	pub density: Density,
}


impl CellData {
	pub fn new(behaviour: CellBehaviour, color: Color, density: u16) -> Self {
		Self { behaviour, color, density }
	}

}
