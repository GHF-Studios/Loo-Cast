//! Built-in first-pass semantic phenomenon models.
//!
//! These are domain implementations of the generic [`PhenomenonRule`] contract.
//! Registry/storage machinery remains independent from this catalogue.

use std::any::Any;

use super::{
    model::{
        COSMIC_MATTER_DISTRIBUTION, COSMOLOGICAL_BACKGROUND, ECOLOGY, GALAXY_INTERSTELLAR_MEDIUM,
        GEOLOGY_CLIMATE_HYDROLOGY, HALO_GALAXY_ENVIRONMENT, MATERIAL_SUBSTRATE, PLANETARY_BODY,
        PhenomenonEvaluationContext, PhenomenonId, PhenomenonSnapshot, STELLAR_SYSTEM_ENVIRONMENT,
        WorldgenNode,
    },
    phenomenon::{PhenomenonRegistry, PhenomenonRule},
    seed::{signed_noise, unit_noise},
};

mod cosmological_background;
mod cosmic_matter_distribution;
mod halo_galaxy_environment;
mod galaxy_interstellar_medium;
mod stellar_system_environment;
mod planetary_body;
mod geology_climate_hydrology;
mod ecology;
mod material_substrate;

pub use cosmological_background::CosmologicalBackgroundState;
pub use cosmic_matter_distribution::CosmicMatterDistributionState;
pub use halo_galaxy_environment::HaloGalaxyEnvironmentState;
pub use galaxy_interstellar_medium::GalaxyInterstellarMediumState;
pub use stellar_system_environment::StellarSystemEnvironmentState;
pub use planetary_body::PlanetaryBodyState;
pub use geology_climate_hydrology::GeologyClimateHydrologyState;
pub use ecology::EcologyState;
pub use material_substrate::MaterialSubstrateState;

pub(super) fn register_builtin_rules(registry: &mut PhenomenonRegistry) {
    registry.register(cosmological_background::CosmologicalBackgroundRule);
    registry.register(cosmic_matter_distribution::CosmicMatterDistributionRule);
    registry.register(halo_galaxy_environment::HaloGalaxyEnvironmentRule);
    registry.register(galaxy_interstellar_medium::GalaxyInterstellarMediumRule);
    registry.register(stellar_system_environment::StellarSystemEnvironmentRule);
    registry.register(planetary_body::PlanetaryBodyRule);
    registry.register(geology_climate_hydrology::GeologyClimateHydrologyRule);
    registry.register(ecology::EcologyRule);
    registry.register(material_substrate::MaterialSubstrateRule);
}

fn current_state<'a, T: Any>(current: &'a [PhenomenonSnapshot], id: PhenomenonId) -> Option<&'a T> {
    let snapshot = current.iter().find(|snapshot| snapshot.id() == id)?;
    snapshot.state::<T>()
}

fn parent_state<'a, T: Any>(parent: Option<&'a WorldgenNode>, id: PhenomenonId) -> Option<&'a T> {
    parent?.state::<T>(id)
}
