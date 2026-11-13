use crate::{ants::AntsPlugin, core::{PositionProps, taxon::TaxonId}};
use bevy::prelude::*;

#[derive(Component)]
pub struct Colony;

#[derive(Component)]
#[relationship(relationship_target = ColonyMembers)]
pub struct MemberOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = MemberOf)]
pub struct ColonyMembers(Vec<Entity>);

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, Self::init)
			.add_systems(Update, Self::update);
	}
}

impl ColonyPlugin {
	pub fn init(mut cmd: Commands) {
		let colony = cmd
			.spawn((Colony, TaxonId(0), PositionProps { x: 0, y: 0 }))
			.id();
		AntsPlugin::spawn_init_ants(cmd, colony);
	}

	pub fn update() {}
}
