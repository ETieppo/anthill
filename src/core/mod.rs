use bevy::prelude::*;
mod logger;
pub use logger::Logger;
pub mod taxon;
pub mod toxin;
pub use toxin::ToxinPlugin;

pub type HealthPoints = u32;

#[derive(Component)]
pub struct Health(pub HealthPoints);

#[derive(Component)]
pub struct PositionProps {
	pub x: i32,
	pub y: i32,
}
