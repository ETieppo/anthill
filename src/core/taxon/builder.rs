use std::collections::HashMap;

use bevy::input::keyboard::Key::New;
use bevy::prelude::*;

use crate::ants::AntsPlugin;
use crate::bacteria::BacteriaPlugin;
use crate::core::taxon::PrototypesTrait;
use crate::core::taxon::TaxonId;
use crate::core::taxon::TaxonInheritanceProps;
use crate::core::taxon::TaxonProps;
use crate::core::taxon::TaxonRankEnum;
use crate::fungi::FungiPlugin;
use crate::riboviria::RiboviriaPlugin;

// HACK: same structure as catalog
#[derive(Resource, Default)]
pub struct TaxonomyBuilder {
	taxa:     Vec<TaxonProps>,
	resolved: Vec<TaxonInheritanceProps>,
	index:    HashMap<&'static str, TaxonId>,
}

impl TaxonomyBuilder {
	pub fn root(
		&mut self,
		name: &'static str,
		rank: TaxonRankEnum,
		traits: TaxonInheritanceProps,
	) -> TaxonId {
		self.push(
			TaxonProps {
				name: String::from(name),
				rank,
				parent: None,
			},
			traits,
		)
	}

	pub fn child(
		&mut self,
		parent: TaxonId,
		name: &'static str,
		rank: TaxonRankEnum,
		traits: TaxonInheritanceProps,
	) -> TaxonId {
		let inherited = traits.inherit_from_parent(self.resolved[parent.0 as usize].clone());

		let id = self.push(
			TaxonProps {
				name:String::from(name),
				rank,
				parent: Some(parent),
			},
			inherited,
		);
		self.index.insert(name, id);
		id
	}

	pub fn build(mut self) -> Self {
		let eukaryota = self.root("Eukaryota", TaxonRankEnum::Domain, default());
		let animalia = self.child(
			eukaryota,
			"Animalia",
			TaxonRankEnum::Kingdom,
			TaxonInheritanceProps {
				base_health: Some(100),
				..default()
			},
		);
		let arthropoda = self.child(animalia, "Arthropoda", TaxonRankEnum::Phylum, default());
		let insecta = self.child(arthropoda, "Insecta", TaxonRankEnum::Class, default());
		let hymenoptera = self.child(insecta, "Hymenoptera", TaxonRankEnum::Order, default());

		FungiPlugin::prototypes(&mut self, Some(eukaryota));
		AntsPlugin::prototypes(&mut self, Some(hymenoptera));
		BacteriaPlugin::prototypes(&mut self, None);
		RiboviriaPlugin::prototypes(&mut self, None);

		self
	}

	pub fn push(&mut self, taxon: TaxonProps, resolved: TaxonInheritanceProps) -> TaxonId {
		let id = TaxonId(self.taxa.len() as u32);
		let new_name: String = if taxon.rank == TaxonRankEnum::Species
			&& let Some(parent_id) = taxon.parent
		{
			Self::mount_name(self.get_by_id(parent_id).name.clone(), taxon.name)
		}
		else {
			taxon.name.into()
		};
		let taxon = TaxonProps {
			name: new_name,
			..taxon
		};
		self.taxa.push(taxon);
		self.resolved.push(resolved);
		id
	}

	/// only for init statements
	pub fn get_taxon_id_by_name(&self, name: &'static str) -> TaxonId {
		match self.index.get(name) {
			Some(id) => Clone::clone(id),
			_ => panic!("[ERR]: no TaxonId with name {}", name),
		}
	}

	pub fn get_by_id(&self, id: TaxonId) -> &TaxonProps { &self.taxa[id.0 as usize] }

	pub fn ancestors(&self, id: TaxonId) -> impl Iterator<Item = TaxonId> + '_ {
		std::iter::successors(self.get_by_id(id).parent, |&current| {
			self.get_by_id(current).parent
		})
	}

	pub fn descends_from(&self, id: TaxonId, ancestor: TaxonId) -> bool {
		id == ancestor || self.ancestors(id).any(|current| current == ancestor)
	}

	fn mount_name(genus: String, species: String) -> String {
		let short: char = genus
			.chars()
			.next()
			.expect("No genus has been defined for species {species}");
		format!("{}. {}", short, species)
	}
}
