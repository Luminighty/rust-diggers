mod input;
mod error;

pub use error::AppError;
pub use input::{ButtonState, Input, key::Key};
use winit::event::{Event, WindowEvent};

use crate::{game, UPSCALE};

pub struct App {
  is_running: bool,
  game_state: game::GameState,
  main_system: Box<dyn game::System<game::GameState>>,
  pixels: pixels::Pixels,
  window: winit::window::Window,
}

impl App {
  pub fn new(window: winit::window::Window, pixels: pixels::Pixels) -> Self {
    Self {
      game_state: game::GameState::new(),
      main_system: game::main_system(),
      is_running: true,
      window, pixels,
    }
  }

  pub fn init(&mut self) {
    self.main_system.init(&mut self.game_state);
  }

  pub fn is_running(&self) -> bool {
    self.is_running
  }

  pub fn handle_event(&mut self, event: Event<'_, ()>) -> Result<(), AppError> {
    match event {
			Event::WindowEvent { event: WindowEvent::CloseRequested, ..} =>
        self.exit(),
			Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => 
        self.cursor_moved(position),
			Event::WindowEvent { event: WindowEvent::MouseInput { state, button, .. }, .. } =>
        self.mouse_input(button, state),
			Event::WindowEvent { event: WindowEvent::KeyboardInput { input,  .. }, ..} =>
        self.keyboard_input(input),
			Event::MainEventsCleared => 
        self.update(),
			Event::RedrawRequested(_) =>
        self.draw()?,
			_ => {}
		}
    Ok(())
  }

  pub fn exit(&mut self) {
    println!("Goodnight");
    self.is_running = false;
  }

  fn cursor_moved(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
    self.game_state.input.set_cursor((position.x / UPSCALE) as usize,(position.y / UPSCALE) as usize)
  }

  fn mouse_input(&mut self, button: winit::event::MouseButton, state: winit::event::ElementState) {
    self.game_state.input.mouse_input(button, state)
  }

  fn keyboard_input(&mut self, input: winit::event::KeyboardInput) {
    self.game_state.input.keyboard_input(input);
  }

  fn update(&mut self) {
    self.main_system.update(&mut self.game_state);
    self.game_state.input.update();
    self.is_running = self.game_state.is_running;
    self.window.request_redraw();
  }

  fn draw(&mut self) -> Result<(), AppError> {
    game::render(&self.game_state, self.pixels.frame_mut());
    Ok(self.pixels.render()?)
  }
}