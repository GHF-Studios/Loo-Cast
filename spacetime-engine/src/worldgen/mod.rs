//! Sparse, demand-driven semantic world generation across USF spatial scales.
//!
//! `worldgen` owns semantic refinement, not runtime realization. It evaluates
//! typed [`PhenomenonRule`]s only for requested branches of the canonical USF
//! hierarchy. Rendering, voxel caches, and other manifestations consume this
//! semantic output but are not authoritative world-generation state.

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
