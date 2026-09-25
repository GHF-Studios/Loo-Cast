//! Experimental sparse rule evaluation across USF spatial scopes.
//!
//! The current builtin catalogue produces descriptive snapshots; it does not
//! construct celestial identities, voxel terrain, ecology, or material state.
//! Loo Cast's authored celestial fixture does not install or consume it.
//!
//! This module evaluates
//! typed [`PhenomenonRule`]s only for requested branches of the canonical USF
//! hierarchy. A future construction adapter must explicitly connect useful
//! output to semantic objects before these snapshots can describe live reality.

mod builtin;
mod model;
mod phenomenon;
mod seed;
mod store;

pub use builtin::{
    CosmicMatterDistributionState, CosmologicalBackgroundState, EcologyState,
    GalaxyInterstellarMediumState, GeologyClimateHydrologyState, HaloGalaxyEnvironmentState,
    MaterialSubstrateState, PlanetaryBodyState, StellarSystemEnvironmentState,
};
pub use model::{
    COSMIC_MATTER_DISTRIBUTION, COSMOLOGICAL_BACKGROUND, DEFAULT_UNIVERSE_SEED, ECOLOGY,
    GALAXY_INTERSTELLAR_MEDIUM, GEOLOGY_CLIMATE_HYDROLOGY, HALO_GALAXY_ENVIRONMENT,
    MATERIAL_SUBSTRATE, PLANETARY_BODY, PhenomenonEvaluationContext, PhenomenonId,
    PhenomenonSnapshot, STELLAR_SYSTEM_ENVIRONMENT, TemporalScale, WorldgenEpoch, WorldgenEpochId,
    WorldgenEvaluationKey, WorldgenNode,
};
pub use phenomenon::{PhenomenonRegistry, PhenomenonRule};
pub use store::{WorldGenerationPlugin, WorldgenStore};

#[cfg(test)]
mod tests;
