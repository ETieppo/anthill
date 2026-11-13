use crate::core::taxon::TaxonId;
use crate::core::taxon::TaxonomyBuilder;

pub trait PrototypesTrait {
	fn prototypes(
		tax_builder: &mut TaxonomyBuilder,
		parent_id: Option<TaxonId>,
	) -> &mut TaxonomyBuilder;
}
