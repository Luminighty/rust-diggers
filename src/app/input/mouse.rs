use super::button::ButtonState;

pub struct Mouse {
  pub left: ButtonState,
  pub middle: ButtonState,
  pub right: ButtonState,
  pub position: (usize, usize),
}

impl Mouse {
  pub fn new() -> Self {
    Self {
      left: ButtonState::Idle,
      middle: ButtonState::Idle,
      right: ButtonState::Idle,
      position: (0, 0),
    }
  }

  pub fn update(&mut self) {
    self.left.update();
    self.middle.update();
    self.right.update();
  }

  pub fn x(&self) -> usize {
    self.position.0
  }
  
  pub fn y(&self) -> usize {
    self.position.1
  }
}