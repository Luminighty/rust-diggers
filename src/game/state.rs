use crate::app::Input;

use super::automata::{World, Cell};

pub struct GameState {
  pub input: Input,
  pub tick: usize,
  pub is_running: bool,
  pub world: World,
  pub selected_cell_type: Cell,

}

impl GameState {
  pub fn new() -> Self {
    Self {
      input: Input::new(),
      tick: 0,
      is_running: true,
      world: World::new(),
      selected_cell_type: Cell::Sand,
    }
  }
}
