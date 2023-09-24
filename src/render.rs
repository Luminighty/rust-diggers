
use crate::automata::{World, Cell};

pub fn render(world: &World, screen: &mut [u8]) {
  for (cell, pixel) in world.cells.iter().zip(screen.chunks_exact_mut(4)) {
    let color = get_color(cell);
    pixel.copy_from_slice(&color);
  }
}

fn get_color(cell: &Cell) -> [u8; 4] {
  match cell {
    Cell::Air => [0, 0, 0, 0xff],
    Cell::Sand => [0xff, 0xbb, 0x00, 0xff],
    Cell::Rock => [0x55, 0x55, 0x77, 0xff],
    Cell::Water => [0x30, 0x90, 0xbb, 0xff],
  }
}