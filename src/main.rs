mod render;
mod automata;

use automata::{World, Cell};
use pixels::{self, SurfaceTexture, Pixels, Error};

use rand::thread_rng;
use winit::{
	dpi::{LogicalSize, PhysicalPosition},
	event::{Event, VirtualKeyCode, WindowEvent, ElementState},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};


pub const WIDTH: u32 = 300;
pub const HEIGHT: u32 = 300;
const UPSCALE: f64 = 2.0;

fn main() -> Result<(), Error>  {

	let event_loop = EventLoop::new();
	let window = {
			let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
			let scaled_size = LogicalSize::new(WIDTH as f64 * UPSCALE, HEIGHT as f64 * UPSCALE);
			WindowBuilder::new()
					.with_title("Sand")
					.with_inner_size(scaled_size)
					.with_min_inner_size(size)
					.build(&event_loop)
					.unwrap()
	};

	let mut world = World::new(WIDTH as isize, HEIGHT as isize);

	let mut pixels = {
			let window_size = window.inner_size();
			let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
			Pixels::new(WIDTH, HEIGHT, surface_texture)?
	};

	let mut tick: usize = 0;
	let mut mouse = PhysicalPosition::new(0, 0);
	let mut mouse_state = ElementState::Released;
	let mut selected_cell = Cell::Sand;
	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => {
				println!("The close button was pressed; stopping");
				control_flow.set_exit();
			}
			Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
				mouse = PhysicalPosition::new((position.x / UPSCALE) as i32, (position.y / UPSCALE) as i32);
			}
			Event::WindowEvent { event: WindowEvent::MouseInput { state, .. }, .. } => {
				mouse_state = state;
			}
			Event::WindowEvent { event: WindowEvent::KeyboardInput { input,  .. }, ..} => {
				if input.state == ElementState::Released {
					return;
				}
				match input.virtual_keycode {
					Some(VirtualKeyCode::F1) => { selected_cell = Cell::Air; },
					Some(VirtualKeyCode::F2) => { selected_cell = Cell::Rock; },
					Some(VirtualKeyCode::F3) => { selected_cell = Cell::Sand; },
					Some(VirtualKeyCode::F4) => { selected_cell = Cell::Water; },
					Some(VirtualKeyCode::F5) => { selected_cell = Cell::Smoke; },
					Some(VirtualKeyCode::F6) => { selected_cell = Cell::Oil; },
					Some(VirtualKeyCode::F7) => { selected_cell = Cell::Slime; },
					Some(VirtualKeyCode::F8) => { selected_cell = Cell::Dirt; },
					_ => {}
				}
			}
			Event::MainEventsCleared => {
				if mouse_state == ElementState::Pressed {
					const BRUSH: isize = 4;
					for brush_x in -BRUSH..BRUSH {
						for brush_y in -BRUSH..BRUSH {
							world.set_cell(brush_x + mouse.x as isize, brush_y + mouse.y as isize, selected_cell)
						}	
					}
				}
				automata::step(&mut world);
				window.request_redraw();
				tick += 1;
			},
			Event::RedrawRequested(_) => {
				render::render(&world, pixels.frame_mut());
				if let Err(err) = pixels.render() {
					println!("Error: {}", err);
					*control_flow = ControlFlow::Exit;
					return;
				}
			},
			_ => ()
		}
	});
}
