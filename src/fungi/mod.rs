use bevy::prelude::*;

use crate::core::taxon::PrototypesTrait;
use crate::core::taxon::TaxonId;

pub struct FungiPlugin;

impl Plugin for FungiPlugin {
	fn build(&self, app: &mut App) {
	}
}

impl PrototypesTrait for FungiPlugin {
	fn prototypes(
		tax_builder: &mut crate::core::taxon::TaxonomyBuilder,
		_parent_id: Option<TaxonId>,
	) -> &mut crate::core::taxon::TaxonomyBuilder {
		// TODO: add some types
		tax_builder
	}
}
