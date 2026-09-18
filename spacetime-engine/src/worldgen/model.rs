//! Core world-generation identities and generated-node model.

use std::any::Any;

use crate::spatial::{SpatialScale, UsfChunkAddress};

pub const DEFAULT_UNIVERSE_SEED: u64 = 0x10_0CA57_5EED_2026;

/// Opaque temporal-model resolution label.
///
/// This is intentionally not yet defined as "seconds per tick". A temporal
/// scale selects a Phenomenon's parameterization/model resolution; future time
/// work can attach timestep/integrator semantics without changing the worldgen
/// evaluation contract introduced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TemporalScale(i16);

impl TemporalScale {
    /// First frozen present-day world-generation parameterization.
    pub const WORLDGEN_SNAPSHOT: Self = Self(0);

    pub const fn new(label: i16) -> Self {
        Self(label)
    }

    pub const fn label(self) -> i16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldgenEpochId(u64);

impl WorldgenEpochId {
    pub const PRESENT_DAY_BOOTSTRAP: Self = Self(1);
}

/// Coarse epoch metadata supplied to every Phenomenon evaluation.
#[derive(Debug, Clone, Copy)]
pub struct WorldgenEpoch {
    id: WorldgenEpochId,
    age_gyr: f64,
    scale_factor: f64,
}

impl WorldgenEpoch {
    pub const fn present_day_bootstrap() -> Self {
        Self {
            id: WorldgenEpochId::PRESENT_DAY_BOOTSTRAP,
            age_gyr: 13.8,
            scale_factor: 1.0,
        }
    }

    pub const fn id(self) -> WorldgenEpochId {
        self.id
    }

    pub const fn age_gyr(self) -> f64 {
        self.age_gyr
    }

    pub const fn scale_factor(self) -> f64 {
        self.scale_factor
    }
}

/// Stable identity for one Phenomenon family/state type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhenomenonId(&'static str);

impl PhenomenonId {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn name(self) -> &'static str {
        self.0
    }
}

pub const COSMOLOGICAL_BACKGROUND: PhenomenonId = PhenomenonId::new("cosmological_background");
pub const COSMIC_MATTER_DISTRIBUTION: PhenomenonId =
    PhenomenonId::new("cosmic_matter_distribution");
pub const HALO_GALAXY_ENVIRONMENT: PhenomenonId = PhenomenonId::new("halo_galaxy_environment");
pub const GALAXY_INTERSTELLAR_MEDIUM: PhenomenonId =
    PhenomenonId::new("galaxy_interstellar_medium");
pub const STELLAR_SYSTEM_ENVIRONMENT: PhenomenonId =
    PhenomenonId::new("stellar_system_environment");
pub const PLANETARY_BODY: PhenomenonId = PhenomenonId::new("planetary_body");
pub const GEOLOGY_CLIMATE_HYDROLOGY: PhenomenonId = PhenomenonId::new("geology_climate_hydrology");
pub const ECOLOGY: PhenomenonId = PhenomenonId::new("ecology");
pub const MATERIAL_SUBSTRATE: PhenomenonId = PhenomenonId::new("material_substrate");

/// Type-erased state emitted by one independently registered Phenomenon rule.
///
/// The store deliberately does not use a global mega-enum for all world state.
/// Callers can recover typed states by Phenomenon id, while additional domains
/// can register new state types without changing the storage model.
pub struct PhenomenonSnapshot {
    id: PhenomenonId,
    state: Box<dyn Any + Send + Sync>,
    summary: String,
}

impl PhenomenonSnapshot {
    pub fn new<T: Any + Send + Sync>(
        id: PhenomenonId,
        state: T,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id,
            state: Box::new(state),
            summary: summary.into(),
        }
    }

    pub const fn id(&self) -> PhenomenonId {
        self.id
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn state<T: Any>(&self) -> Option<&T> {
        self.state.as_ref().downcast_ref::<T>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldgenEvaluationKey {
    scope: UsfChunkAddress,
    temporal_scale: TemporalScale,
    epoch: WorldgenEpochId,
}

impl WorldgenEvaluationKey {
    pub const fn scope(self) -> UsfChunkAddress {
        self.scope
    }

    pub const fn temporal_scale(self) -> TemporalScale {
        self.temporal_scale
    }

    pub const fn epoch(self) -> WorldgenEpochId {
        self.epoch
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhenomenonEvaluationContext {
    key: WorldgenEvaluationKey,
    epoch: WorldgenEpoch,
    seed: u64,
}

impl PhenomenonEvaluationContext {
    pub const fn key(self) -> WorldgenEvaluationKey {
        self.key
    }

    pub const fn spatial_scope(self) -> UsfChunkAddress {
        self.key.scope
    }

    pub const fn spatial_scale(self) -> SpatialScale {
        self.key.scope.scale()
    }

    pub const fn temporal_scale(self) -> TemporalScale {
        self.key.temporal_scale
    }

    pub const fn epoch(self) -> WorldgenEpoch {
        self.epoch
    }

    pub const fn seed(self) -> u64 {
        self.seed
    }
}

/// One generated semantic scope at one temporal parameterization and epoch.
pub struct WorldgenNode {
    context: PhenomenonEvaluationContext,
    parent: Option<WorldgenEvaluationKey>,
    phenomena: Vec<PhenomenonSnapshot>,
}

impl WorldgenNode {
    pub const fn context(&self) -> PhenomenonEvaluationContext {
        self.context
    }

    pub const fn key(&self) -> WorldgenEvaluationKey {
        self.context.key
    }

    pub const fn parent(&self) -> Option<WorldgenEvaluationKey> {
        self.parent
    }

    pub fn phenomena(&self) -> impl ExactSizeIterator<Item = &PhenomenonSnapshot> {
        self.phenomena.iter()
    }

    pub fn state<T: Any>(&self, id: PhenomenonId) -> Option<&T> {
        let snapshot = self.phenomena.iter().find(|snapshot| snapshot.id == id)?;
        snapshot.state::<T>()
    }
}
