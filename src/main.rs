mod app;
mod game;

use app::AppError;
use pixels::{self, SurfaceTexture, Pixels};

const TITLE: &str = "Sand";

pub const WIDTH: isize = 100;
pub const HEIGHT: isize = 100;
const UPSCALE: f64 = 3.0;

const SCALED_WIDTH: f64 = WIDTH as f64 * UPSCALE;
const SCALED_HEIGHT: f64 = HEIGHT as f64 * UPSCALE;

fn main() -> Result<(), AppError>  {
	let event_loop = winit::event_loop::EventLoop::new();
	let mut app = init_app(&event_loop)?;

	app.init();
	event_loop.run(move |event: winit::event::Event<'_, ()>, _, control_flow| {
		app.handle_event(event).unwrap();
		if !app.is_running() {
			control_flow.set_exit();
		}
	});
}

fn init_app(event_loop: &winit::event_loop::EventLoop<()>) -> Result<app::App, AppError> {
	let window = winit::window::WindowBuilder::new()
		.with_title(TITLE)
		.with_inner_size(winit::dpi::LogicalSize::new(SCALED_WIDTH, SCALED_HEIGHT))
		.with_min_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
		.with_resizable(false)
		.build(&event_loop)
		.unwrap();

	let pixels = {
			let window_size = window.inner_size();
			let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
			Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
	};
	Ok(app::App::new(window, pixels))
}