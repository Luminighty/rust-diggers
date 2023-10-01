pub trait System<T> {
  fn init(&self, _data: &mut T) {}
  fn update(&self, _data: &mut T) {}
}

pub struct SystemCollection<T> {
  systems: Vec<Box<dyn System<T>>>
}

impl<T> SystemCollection<T> {
  pub fn new(systems: Vec<Box<dyn System<T>>>) -> Self {
    Self { systems }
  }
}

impl<T> System<T> for SystemCollection<T> {
  fn init(&self, data: &mut T) {
    for system in &self.systems {
      system.init(data)
    }
  }

  fn update(&self, data: &mut T) {
    for system in &self.systems {
      system.update(data)
    }
  }
}
