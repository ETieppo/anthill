mod ants;
mod bacteria;
mod colony;
mod core;
mod fungi;
mod riboviria;

use bevy::prelude::*;
pub use core::Logger;

fn main() {
	App::new()
		.add_plugins(MinimalPlugins)
		.add_plugins(core::ToxinPlugin)
		.add_plugins(core::taxon::TaxonPlugin)
		.add_plugins(colony::ColonyPlugin)
		.add_plugins(ants::AntsPlugin)
		.run();
}
