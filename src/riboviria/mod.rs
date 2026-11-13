use bevy::prelude::*;

use crate::core::taxon::PrototypesTrait;
use crate::core::taxon::TaxonRankEnum;
use crate::core::taxon::TaxonId;

pub struct RiboviriaPlugin;

impl Plugin for RiboviriaPlugin {
	fn build(&self, app: &mut App) {
	}
}

impl PrototypesTrait for RiboviriaPlugin {
	fn prototypes(
		tax_builder: &mut crate::core::taxon::TaxonomyBuilder,
		_parent_id: Option<TaxonId>,
	) -> &mut crate::core::taxon::TaxonomyBuilder {
		tax_builder.root("Riboviria", TaxonRankEnum::Realm, default());
		tax_builder
	}
}
