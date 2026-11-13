mod prototypes;
use bevy::prelude::*;

use crate::ants::prototypes::SOLENOPSIS_INVICTA_NAME;
use crate::colony::MemberOf;
use crate::core::taxon::PrototypesTrait;
use crate::core::taxon::TaxonId;
use crate::core::taxon::TaxonomyBuilder;

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
			.add_systems(Update, Self::update);
	}
}

impl AntsPlugin {
	pub fn init(mut cmd: Commands, res: Res<TaxonomyBuilder>) {
		let proto_id = res.get_taxon_id_by_name(SOLENOPSIS_INVICTA_NAME);
		cmd.spawn((Ant, QueenAnt, proto_id));
	}

	pub fn update(
		mut cmd: Commands,
		ants: Query<(Entity, &Ant, &TaxonId)>,
		taxons: Res<TaxonomyBuilder>,
	) {
		for ant in ants {
			let taxon = taxons.ancestors(*ant.2);
			for ac in taxon {
				println!("{}", ac.0);
			}
		}
	}

	pub fn spawn_init_ants(mut cmd: Commands, colony: Entity) {
		cmd.spawn((Ant, MemberOf(colony), QueenAnt));
		cmd.spawn((Ant, MemberOf(colony), NaniticAnt));
	}
}
