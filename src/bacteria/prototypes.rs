use bevy::prelude::*;

use super::BacteriaPlugin;
use crate::core::taxon::PrototypesTrait;
use crate::core::taxon::TaxonId;
use crate::core::taxon::TaxonRankEnum;
use crate::core::taxon::TaxonomyBuilder;
use crate::core::taxon::TaxonInheritanceProps;
use crate::core::toxin::ToxinPrototypes;

impl PrototypesTrait for BacteriaPlugin {
	fn prototypes(
		tax_builder: &mut TaxonomyBuilder,
		_parent: Option<TaxonId>,
	) -> &mut TaxonomyBuilder {
		// TODO: add some types
		let bacteria = tax_builder.root("Bacteria", TaxonRankEnum::Domain, TaxonInheritanceProps {
			base_health: Some(1),
			..default()
		});
		let firmicutes = tax_builder.child(bacteria, "Firmicutes", TaxonRankEnum::Phylum, default());
		let bacilli = tax_builder.child(firmicutes, "Bacilli", TaxonRankEnum::Class, default());
		let lactobacillales =
			tax_builder.child(bacilli, "Lactobacillales", TaxonRankEnum::Order, default());
		let streptococcaceae = tax_builder.child(
			lactobacillales,
			"Streptococcaceae",
			TaxonRankEnum::Family,
			default(),
		);
		let streptococcus = tax_builder.child(
			streptococcaceae,
			"Streptococcus",
			TaxonRankEnum::Genus,
			default(),
		);
		tax_builder.child(streptococcus, "Pyogenes", TaxonRankEnum::Species, TaxonInheritanceProps {
			toxins: vec![
				ToxinPrototypes::ESTREPTOLISIN,
				ToxinPrototypes::PIROGENIC_EXOTOXIN,
			],
			..default()
		});
		tax_builder
	}
}
