use crate::core::HealthPoints;
use crate::core::toxin::ToxinId;

#[derive(Debug, Clone, Default)]
pub struct TaxonInheritanceProps {
	pub base_health: Option<HealthPoints>,
	pub toxins:      Vec<ToxinId>,
}

impl TaxonInheritanceProps {
	pub fn inherit_from_parent(mut self, parent: Self) -> Self {
		self.toxins.extend(parent.toxins);

		TaxonInheritanceProps {
			base_health: self.base_health.or(parent.base_health),
			toxins:      self.toxins,
		}
	}
}
