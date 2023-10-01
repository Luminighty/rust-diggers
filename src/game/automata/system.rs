use std::{borrow::BorrowMut, ops::DerefMut};

use rand::{seq::SliceRandom, thread_rng};

use crate::{game::{GameState, System}, HEIGHT, WIDTH, app::{Input, ButtonState, Key}};

use super::{World, Cell};


pub struct AutomataSystem;
impl System<GameState> for AutomataSystem {

  fn init(&self, game_state: &mut GameState) {
    for i in 0..1 {
      let x = 50 + i % 10;
      let y = 50 + i / 10;
      if !is_inbound(x, y) {
        continue;
      }
      game_state.world.set_cell(x, y, Cell::Sand.build());
    }
    println!("init automata...");
  }

  fn update(&self, game_state: &mut GameState) {
    palette_select(game_state);
    god_mode(&game_state.input, &mut game_state.world, &game_state.selected_cell_type);
    for i in 0..game_state.world.cells.len() {
      update_cell(&mut game_state.world, i);
    }
    apply_actions(&mut game_state.world);
  }
}

fn palette_select(game_state: &mut GameState) {
  if game_state.input.key(Key::Key1) == &ButtonState::Released {
    game_state.selected_cell_type = Cell::Sand;
  }
  if game_state.input.key(Key::Key2) == &ButtonState::Released {
    game_state.selected_cell_type = Cell::Rock;
  }
  if game_state.input.key(Key::Key3) == &ButtonState::Released {
    game_state.selected_cell_type = Cell::Water;
  }
  if game_state.input.key(Key::Key0) == &ButtonState::Released {
    game_state.selected_cell_type = Cell::Air;
  }
  if game_state.input.key(Key::Key4) == &ButtonState::Released {
    game_state.selected_cell_type = Cell::Smoke;
  }
  if game_state.input.key(Key::Key5) == &ButtonState::Released {
    game_state.selected_cell_type = Cell::Slime;
  }
  if game_state.input.key(Key::Key6) == &ButtonState::Released {
    game_state.selected_cell_type = Cell::Dirt;
  }
}

const BRUSH: isize = 3;
fn god_mode(input: &Input, world: &mut World, cell: &Cell) {
  if !input.mouse.left.is_pressing() {
    return
  }
  for brush_x in -BRUSH..BRUSH {
    for brush_y in -BRUSH..BRUSH {
      let x = brush_x + input.mouse.x() as isize;
      let y = brush_y + input.mouse.y() as isize;
      if !is_inbound(x, y) {
        continue;
      }
      world.set_cell(x, y, cell.build());
      awake_neighbours(world, x, y);
    }
  }
}

fn apply_actions(world: &mut World) {
  world.actions.actions.shuffle(&mut thread_rng());
  for i in 0..world.actions.actions.len() {
    let ((to_x, to_y), (from_x, from_y)) = world.actions.actions[i];
    world.move_cell(from_x, from_y, to_x, to_y);
    awake_neighbours(world, from_x, from_y);
    awake_neighbours(world, to_x, to_y);
  }
  world.actions.clear();
}

fn awake_neighbours(world: &mut World, x: isize, y: isize) {
  world.set_sleep(x - 1, y - 1, false);
  world.set_sleep(x - 1, y, false);
  world.set_sleep(x - 1, y + 1, false);

  world.set_sleep(x + 1, y - 1, false);
  world.set_sleep(x + 1, y, false);
  world.set_sleep(x + 1, y + 1, false);

  world.set_sleep(x, y - 1, false);
  world.set_sleep(x, y + 1, false);
}

fn update_cell(world: &mut World, i: usize) {
  if world.cells[i].is_none() {
    return;
  }
  let binding = world.cells[i].clone().unwrap();
  let cell = binding.borrow();
  if cell.is_sleeping {
    return;
  }
  for (delta_x, delta_y) in cell.behaviour().steps() {
    let from_x = i as isize % WIDTH;
    let from_y = i as isize / WIDTH;
    let to_x = from_x + delta_x;
    let to_y = from_y + delta_y;
    if !is_inbound(to_x, to_y) {
      continue;
    }
    if let Some(other) = world.get_cell(to_x, to_y) {
      if other.borrow().density() >= cell.density() {
        continue;
      }
    }
    world.actions.move_cell(from_x, from_y, to_x, to_y);
    return;
  }
  drop(cell);

  let mut cell = (*binding).borrow_mut();
  cell.is_sleeping = true;
}

fn is_inbound(x: isize, y: isize) -> bool {
  x < WIDTH as isize && x >= 0 && y < HEIGHT as isize && y >= 0
}
