use crate::{automata::{World, CellData, cell}, WIDTH, HEIGHT};

pub fn render(world: &World, screen: &mut [u8]) {
  for (i, pixel) in (0..WIDTH*HEIGHT).zip(screen.chunks_exact_mut(4)) {
    let x = i % WIDTH;
    let y = i / WIDTH;
    let color = get_color(world.get_cell(x as isize, y as isize));
    pixel.copy_from_slice(&color);
  }
}

fn get_color(cell: Option<CellData>) -> [u8; 4] {
  if let Some(cell) = cell {
    cell.color
  } else {
    [0, 0, 0, 0]
  }
}
