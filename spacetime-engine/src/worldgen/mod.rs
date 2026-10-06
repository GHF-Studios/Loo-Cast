//! Sparse typed phenomenon evaluation across canonical USF spatial scopes.
//!
//! The current builtin catalogue produces descriptive snapshots; it does not
//! construct celestial identities, voxel terrain, ecology, or material state.
//! Loo Cast's authored celestial fixture does not install or consume it.
//!
//! This module evaluates
//! typed [`PhenomenonRule`]s only for requested branches of the canonical USF
//! hierarchy. A future construction adapter must explicitly connect useful
//! output to semantic objects before these snapshots can describe live reality.
//!
//! ## Integration
//!
//! Evaluation produces requested phenomenon facts. A separate authored construction adapter decides
//! whether those facts create identities, terrain, or gameplay state.
//!
//! ## Module map
//!
//! - `builtin`: Built-in first-pass semantic phenomenon models.
//! - `model`: Core world-generation identities and generated-node model.
//! - `phenomenon`: Phenomenon rule contract and registry.
//! - `seed`: Deterministic world-generation hashing/noise helpers.
//! - `store`: Sparse phenomenon-evaluation cache and contextual refinement facility.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

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
    COSMIC_MATTER_DISTRIBUTION, COSMOLOGICAL_BACKGROUND, DEFAULT_WORLDGEN_NOISE_KEY, ECOLOGY,
    GALAXY_INTERSTELLAR_MEDIUM, GEOLOGY_CLIMATE_HYDROLOGY, HALO_GALAXY_ENVIRONMENT,
    MATERIAL_SUBSTRATE, PLANETARY_BODY, PhenomenonEvaluationContext, PhenomenonId,
    PhenomenonSnapshot, STELLAR_SYSTEM_ENVIRONMENT, TemporalScale, WorldgenEpoch, WorldgenEpochId,
    WorldgenEvaluation, WorldgenEvaluationKey,
};
pub use phenomenon::{PhenomenonRegistry, PhenomenonRule};
pub use store::{WorldgenEvaluationCache, WorldgenEvaluationPlugin};
