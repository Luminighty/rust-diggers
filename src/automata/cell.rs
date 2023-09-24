#[derive(Debug, Clone, Copy)]
pub enum Cell {
	Air,
	Sand,
	Water,
	Rock
}

impl Cell {

	pub fn steps(&self) -> std::slice::Iter<'_, (isize, isize)> {
		match self {
			Cell::Sand => [(0, 1), (-1, 1), (1, 1)].iter(),
			Cell::Water => [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)].iter(),
			_ => [].iter()
		}
	}

}