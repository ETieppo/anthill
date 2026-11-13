mod builder;
mod inheritance;
mod props;
mod prototypes;

use bevy::prelude::*;
pub use builder::TaxonomyBuilder;
pub use inheritance::TaxonInheritanceProps;
pub use props::TaxonId;
pub use props::TaxonProps;
pub use props::TaxonRankEnum;
pub use prototypes::PrototypesTrait;

pub struct TaxonPlugin;

impl Plugin for TaxonPlugin {
	fn build(&self, app: &mut App) {
		let builder = TaxonomyBuilder::default();
		app.insert_resource(builder.build());
	}
}
