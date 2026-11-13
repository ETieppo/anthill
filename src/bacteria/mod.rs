mod prototypes;
use bevy::prelude::*;

use crate::core::taxon::PrototypesTrait;
use crate::core::taxon::TaxonId;
use crate::core::taxon::TaxonRankEnum;
use crate::core::taxon::TaxonomyBuilder;
use crate::core::taxon::TaxonInheritanceProps;
use crate::core::toxin::ToxinPrototypes;

pub struct BacteriaPlugin;

impl Plugin for BacteriaPlugin {
	fn build(&self, app: &mut App) {
	}
}
