use crate::{automata::{World, Cell}, WIDTH, HEIGHT};

pub fn render(world: &World, screen: &mut [u8]) {
  for (i, pixel) in (0..WIDTH*HEIGHT).zip(screen.chunks_exact_mut(4)) {
    let x = i % WIDTH;
    let y = i / WIDTH;
    let color = get_color(world.get_cell(x as isize, y as isize));
    pixel.copy_from_slice(&color);
  }
}

fn get_color(cell: Option<Cell>) -> [u8; 4] {
  match cell {
    None => [0, 0, 0, 0xff],
    Some(Cell::Air) => [0, 0, 0, 0xff],
    Some(Cell::Sand) => [0xff, 0xbb, 0x00, 0xff],
    Some(Cell::Rock) => [0x55, 0x55, 0x77, 0xff],
    Some(Cell::Water) => [0x30, 0x90, 0xbb, 0xff],
    Some(Cell::Smoke) => [0x30, 0x30, 0x30, 0xff],
    Some(Cell::Slime) => [0x66, 0x22, 0x66, 0xff],
    Some(Cell::Oil) => [0x88, 0x66, 0x44, 0xff],
    Some(Cell::Dirt) => [0x99, 0x55, 0x22, 0xff],
  }
}