mod cell;
mod world;
mod cell_types;
mod system;

pub use cell::CellData;
pub use cell_types::Cell;
pub use world::{World, StoredCell};
pub use system::AutomataSystem;
