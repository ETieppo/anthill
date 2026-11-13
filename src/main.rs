mod ants;
mod colony;
mod core;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(MinimalPlugins)
		.add_plugins(colony::ColonyPlugin)
		.add_plugins(ants::AntsPlugin)
		.run();
}
