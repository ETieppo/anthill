use bevy::prelude::*;

pub struct ToxinProps {
	pub name:       &'static str,
	pub raw_damage: super::HealthPoints,
}

#[derive(Clone, Copy, Debug)]
pub struct ToxinId(pub u16);

#[derive(Resource)]
pub struct ToxinPrototypes(Vec<ToxinProps>);

pub struct ToxinPlugin;

impl ToxinPrototypes {
	pub const ESTREPTOLISIN: ToxinId = ToxinId(2);
	pub const PIROGENIC_EXOTOXIN: ToxinId = ToxinId(1);
	pub const SOLENOPSIN: ToxinId = ToxinId(0);

	pub fn get_toxin_by_id(&self, id: ToxinId) -> &ToxinProps { &self.0[id.0 as usize] }
}

impl Default for ToxinPrototypes {
	fn default() -> Self {
		ToxinPrototypes(vec![
			ToxinProps {
				name:       "Solenopsin",
				raw_damage: 32,
			},
			ToxinProps {
				name:       "Pirogenic Exotoxin",
				raw_damage: 30,
			},
			ToxinProps {
				name:       "Estreptolisin",
				raw_damage: 50,
			},
		])
	}
}

impl Plugin for ToxinPlugin {
	fn build(&self, app: &mut App) { app.init_resource::<ToxinPrototypes>(); }
}
