use bevy::prelude::*;

pub struct ToxinProps {
	pub name: &'static str,
	pub raw_damage: super::HealthPoints,
}

#[derive(Clone, Copy)]
pub struct ToxinId(pub u16);

#[derive(Resource)]
pub struct ToxinPrototypes(Vec<ToxinProps>);

pub struct ToxinPlugin;

impl ToxinPrototypes {
	pub const SOLENOPSIN: ToxinId = ToxinId(0);

	pub fn get_toxin_by_id(&self, id: ToxinId) -> &ToxinProps {
		&self.0[id.0 as usize]
	}
}

impl Default for ToxinPrototypes {
	fn default() -> Self {
		ToxinPrototypes(vec![ToxinProps {
			name: "Solenopsin",
			raw_damage: 32,
		}])
	}
}

impl Plugin for ToxinPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<ToxinPrototypes>();
	}
}
