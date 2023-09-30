mod system;
mod state;
mod render;
mod automata;

pub use render::render;
pub use system::System;
pub use state::GameState;

pub fn main_system() -> Box<dyn System<GameState>> {
  Box::new(system::SystemCollection::new(vec![
    Box::new(TickerSystem),
    Box::new(automata::AutomataSystem)
  ]))
}

struct TickerSystem;
impl System<GameState> for TickerSystem {
  fn update(&self, game_state: &mut GameState) {
    game_state.tick += 1;
  }
}
