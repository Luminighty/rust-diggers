use crate::{WIDTH, HEIGHT};

use super::{automata::StoredCell, GameState};

type Screen = [u8];

pub fn render(game_state: &GameState, screen: &mut Screen) {
  for (i, pixel) in (0..WIDTH*HEIGHT).zip(screen.chunks_exact_mut(4)) {
    let x = i % WIDTH;
    let y = i / WIDTH;
    let color = get_color(game_state.world.get_cell(x as isize, y as isize));
    pixel.copy_from_slice(&color);
  }
}


fn get_color(cell: Option<&StoredCell>) -> [u8; 4] {
  if let Some(cell) = cell {
    cell.borrow().color
  } else {
    [0, 0, 0, 0]
  }
}
