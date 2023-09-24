pub mod world;
pub mod cell;

pub use world::World;
pub use cell::Cell;

fn simulate_cell(x: isize, y: isize, world: &mut World) {
	if let Some(&cell) = world.get_cell(x, y) {
		for (dx, dy) in (cell).steps() {
			let (target_x, target_y) = (x + dx, y + dy);
			if world.is_empty(x + dx, y + dy) {
				world.move_cell(x, y, target_x, target_y);
				return;
			}
		}
	}
}

pub fn step(world: &mut World) {
	for x in 0..world.width {
		for y in 0..world.height {
			simulate_cell(x, world.height - y - 1, world)
		}
	}
	world.commit_changes();
}
