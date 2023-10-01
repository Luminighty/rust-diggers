#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
  Idle,
  Pressed,
  Held,
  Released,
}

impl ButtonState {
  pub fn update(&mut self) {
    *self = self.next()
  }

  pub fn next(&self) -> Self {
    match self {
      Self::Pressed => Self::Held,
      Self::Released => Self::Idle,
      any => any.clone()
    }
  }

  pub fn is_pressing(&self) -> bool {
    match self {
      Self::Pressed => true,
      Self::Held => true,
      _ => false
    }
  }
}


impl From<winit::event::ElementState> for ButtonState {
  fn from(value: winit::event::ElementState) -> Self {
    match value {
      winit::event::ElementState::Pressed => ButtonState::Pressed,
      winit::event::ElementState::Released => ButtonState::Released,
    }
  }
}
