use bevy::prelude::*;

pub mod poison;
pub mod taxon;

pub type HealthPoints = u32;

#[derive(Component)]
pub struct Health(pub HealthPoints);

#[derive(Component)]
pub struct PositionProps {
	pub x: i32,
	pub y: i32,
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
	fn build(&self, app: &mut App) {
		for t in taxon::TAXA.iter() {
			app.init_resource::<taxon::TaxonProps>();
		}
	}
}
