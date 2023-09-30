use std::collections::HashMap;

use winit::event::{MouseButton, ElementState, KeyboardInput};

pub use self::button::ButtonState;

mod button;
mod mouse;
pub mod key;

pub struct Input {
  pub mouse: mouse::Mouse,
  keys: HashMap<u32, ButtonState>
}

impl Input {
  pub fn new() -> Self {
    Self {
      mouse: mouse::Mouse::new(),
      keys: HashMap::new(),
    }
  }

  pub fn key(&self, key: key::Key) -> &ButtonState {
    if let Some(state) = self.keys.get(&(key as u32)) {
      state
    } else {
      &ButtonState::Idle
    }
  }

  pub fn set_cursor(&mut self, x: usize, y: usize) {
    self.mouse.position = (x, y)
  }

  pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
    match button {
      MouseButton::Left => self.mouse.left = state.into(),
      MouseButton::Middle => self.mouse.middle = state.into(),
      MouseButton::Right => self.mouse.right = state.into(),
      _ => {},
    }
  }

  pub fn keyboard_input(&mut self, input: KeyboardInput) {
    self.keys.insert(input.scancode, input.state.into());
  }

  pub fn update(&mut self) {
    self.mouse.update();
    for key in self.keys.values_mut() {
      key.update();
    }

  }
}