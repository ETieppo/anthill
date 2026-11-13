use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TaxonId(pub u32);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TaxonRankEnum {
	Realm,
	Domain,
	Kingdom,
	Phylum,
	Class,
	Order,
	Family,
	Genus,
	Species,
}

#[derive(Debug)]
pub struct TaxonProps {
	pub name:   String,
	pub rank:   TaxonRankEnum,
	pub parent: Option<TaxonId>,
}
