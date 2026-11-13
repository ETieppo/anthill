use crate::ants::AntsPlugin;
use crate::core::taxon::PrototypesTrait;
use crate::core::taxon::TaxonId;
use crate::core::taxon::TaxonInheritanceProps;
use crate::core::taxon::TaxonRankEnum;
use crate::core::taxon::TaxonomyBuilder;
use crate::core::toxin::ToxinPrototypes;

pub const SOLENOPSIS_INVICTA_NAME: &'static str = "Invicta";

impl PrototypesTrait for AntsPlugin {
	fn prototypes(tax_builder: &mut TaxonomyBuilder, p_id: Option<TaxonId>) -> &mut TaxonomyBuilder {
		let Some(parent_id) = p_id
		else {
			panic!("");
		};
		let default_trait = TaxonInheritanceProps::default();
		let formicidae = tax_builder.child(
			parent_id,
			"Formicidae",
			TaxonRankEnum::Family,
			default_trait.clone(),
		);
		let solenopsis = tax_builder.child(
			formicidae,
			"Solenopsis",
			TaxonRankEnum::Genus,
			TaxonInheritanceProps {
				toxins: [ToxinPrototypes::SOLENOPSIN].into(),
				..default_trait
			},
		);

		tax_builder.child(
			solenopsis,
			SOLENOPSIS_INVICTA_NAME,
			TaxonRankEnum::Species,
			TaxonInheritanceProps {
				base_health: Some(100),
				..default_trait
			},
		);

		tax_builder
	}
}
