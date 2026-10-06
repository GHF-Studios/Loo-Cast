//! Built-in first-pass semantic phenomenon models.
//!
//! These are domain implementations of the generic [`PhenomenonRule`] contract.
//! Registry/storage machinery remains independent from this catalogue.
//!
//! ## Module map
//!
//! - `cosmic_matter_distribution`: Built-in `cosmic_matter_distribution` worldgen phenomenon.
//! - `cosmological_background`: Built-in `cosmological_background` worldgen phenomenon.
//! - `ecology`: Built-in `ecology` worldgen phenomenon.
//! - `galaxy_interstellar_medium`: Built-in `galaxy_interstellar_medium` worldgen phenomenon.
//! - `geology_climate_hydrology`: Built-in `geology_climate_hydrology` worldgen phenomenon.
//! - `halo_galaxy_environment`: Built-in `halo_galaxy_environment` worldgen phenomenon.
//! - `material_substrate`: Built-in `material_substrate` worldgen phenomenon.
//! - `planetary_body`: Built-in `planetary_body` worldgen phenomenon.
//! - `stellar_system_environment`: Built-in `stellar_system_environment` worldgen phenomenon.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use std::any::Any;

use super::{
    model::{
        COSMIC_MATTER_DISTRIBUTION, COSMOLOGICAL_BACKGROUND, ECOLOGY, GALAXY_INTERSTELLAR_MEDIUM,
        GEOLOGY_CLIMATE_HYDROLOGY, HALO_GALAXY_ENVIRONMENT, MATERIAL_SUBSTRATE, PLANETARY_BODY,
        PhenomenonEvaluationContext, PhenomenonId, PhenomenonSnapshot, STELLAR_SYSTEM_ENVIRONMENT,
        WorldgenEvaluation,
    },
    phenomenon::{PhenomenonRegistry, PhenomenonRule},
    seed::{signed_noise, unit_noise},
};

mod cosmic_matter_distribution;
mod cosmological_background;
mod ecology;
mod galaxy_interstellar_medium;
mod geology_climate_hydrology;
mod halo_galaxy_environment;
mod material_substrate;
mod planetary_body;
mod stellar_system_environment;

pub use cosmic_matter_distribution::CosmicMatterDistributionState;
pub use cosmological_background::CosmologicalBackgroundState;
pub use ecology::EcologyState;
pub use galaxy_interstellar_medium::GalaxyInterstellarMediumState;
pub use geology_climate_hydrology::GeologyClimateHydrologyState;
pub use halo_galaxy_environment::HaloGalaxyEnvironmentState;
pub use material_substrate::MaterialSubstrateState;
pub use planetary_body::PlanetaryBodyState;
pub use stellar_system_environment::StellarSystemEnvironmentState;

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

fn parent_state<'a, T: Any>(
    parent: Option<&'a WorldgenEvaluation>,
    id: PhenomenonId,
) -> Option<&'a T> {
    parent?.state::<T>(id)
}
