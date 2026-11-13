use bevy::prelude::*;

use crate::{
	colony::MemberOf,
	core::{poison::ToxinId, taxon::TaxonProps},
};

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct QueenAnt;

#[derive(Component)]
pub struct WorkerAnt;

#[derive(Component)]
pub struct DroneAnt;

#[derive(Component)]
pub struct AlatesAnt;

#[derive(Component)]
pub struct EggAnt;

#[derive(Component)]
pub struct NaniticAnt;

#[derive(Component)]
pub struct LarvaeAnt;

#[derive(Component)]
pub struct PupaeAnt;

pub struct AntsPlugin;

impl Plugin for AntsPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, Self::init)
			.add_systems(Update, Self::update)
			.insert_resource(Self::fire_ant_prototypes());
	}
}

impl AntsPlugin {
	// TODO: fix _cmd
	pub fn init(mut _cmd: Commands) {}

	// TODO: fix _cmd
	pub fn update(mut _cmd: Commands) {}

	pub fn spawn_init_ants(mut cmd: Commands, colony: Entity) {
		cmd.spawn((Ant, MemberOf(colony), QueenAnt));
		cmd.spawn((Ant, MemberOf(colony), NaniticAnt));
	}

	pub fn fire_ant_prototypes() -> Prototypes {
		Prototypes {
			taxa: vec![TaxonProps {
				toxin: ToxinId(0),
				name: "Solenopsis invicta",
				base_health: 100,
			}],
		}
	}
}
