//! Sparse, demand-driven semantic world generation across USF spatial scales.
//!
//! This module owns semantic refinement, not runtime realization. It starts from
//! a coarse present-day bootstrap at Scale +35 and evaluates registered
//! Phenomenon rules down the canonical hierarchy only where a branch is needed.

use std::{any::Any, collections::HashMap};

use bevy::prelude::*;

use crate::spatial::{
    SPATIAL_SCALE_MAX, SpatialScale, UsfChunkAddress, UsfPosition, UsfPositionError,
};

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

pub const COSMOLOGICAL_BACKGROUND: PhenomenonId =
    PhenomenonId::new("cosmological_background");
pub const COSMIC_MATTER_DISTRIBUTION: PhenomenonId =
    PhenomenonId::new("cosmic_matter_distribution");
pub const HALO_GALAXY_ENVIRONMENT: PhenomenonId =
    PhenomenonId::new("halo_galaxy_environment");
pub const GALAXY_INTERSTELLAR_MEDIUM: PhenomenonId =
    PhenomenonId::new("galaxy_interstellar_medium");
pub const STELLAR_SYSTEM_ENVIRONMENT: PhenomenonId =
    PhenomenonId::new("stellar_system_environment");
pub const PLANETARY_BODY: PhenomenonId = PhenomenonId::new("planetary_body");
pub const GEOLOGY_CLIMATE_HYDROLOGY: PhenomenonId =
    PhenomenonId::new("geology_climate_hydrology");
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

/// A self-contained domain rule that can continue/refine state or introduce a
/// new Phenomenon when its spatial domain becomes meaningful.
pub trait PhenomenonRule: Send + Sync + 'static {
    fn id(&self) -> PhenomenonId;

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot>;
}

/// Ordered only for same-scope handoff dependencies in this first implementation.
/// The states themselves remain independently typed and registered.
#[derive(Resource)]
pub struct PhenomenonRegistry {
    rules: Vec<Box<dyn PhenomenonRule>>,
}

impl Default for PhenomenonRegistry {
    fn default() -> Self {
        let mut registry = Self { rules: Vec::new() };
        registry.register(CosmologicalBackgroundRule);
        registry.register(CosmicMatterDistributionRule);
        registry.register(HaloGalaxyEnvironmentRule);
        registry.register(GalaxyInterstellarMediumRule);
        registry.register(StellarSystemEnvironmentRule);
        registry.register(PlanetaryBodyRule);
        registry.register(GeologyClimateHydrologyRule);
        registry.register(EcologyRule);
        registry.register(MaterialSubstrateRule);
        registry
    }
}

impl PhenomenonRegistry {
    pub fn register<R: PhenomenonRule>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
    ) -> Vec<PhenomenonSnapshot> {
        let mut snapshots = Vec::new();
        for rule in &self.rules {
            if let Some(snapshot) = rule.evaluate(context, parent, &snapshots) {
                debug_assert_eq!(snapshot.id(), rule.id());
                snapshots.push(snapshot);
            }
        }
        snapshots
    }
}

/// Sparse semantic generation cache. Addressability of the USF hierarchy does
/// not imply entries here: only requested/refined scopes are allocated.
#[derive(Resource)]
pub struct WorldgenStore {
    universe_seed: u64,
    nodes: HashMap<WorldgenEvaluationKey, WorldgenNode>,
}

impl Default for WorldgenStore {
    fn default() -> Self {
        Self {
            universe_seed: DEFAULT_UNIVERSE_SEED,
            nodes: HashMap::new(),
        }
    }
}

impl WorldgenStore {
    pub fn new(universe_seed: u64) -> Self {
        Self {
            universe_seed,
            nodes: HashMap::new(),
        }
    }

    pub const fn universe_seed(&self) -> u64 {
        self.universe_seed
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn key_for(
        &self,
        position: UsfPosition,
        scale: SpatialScale,
        temporal_scale: TemporalScale,
        epoch: WorldgenEpoch,
    ) -> Result<WorldgenEvaluationKey, UsfPositionError> {
        Ok(WorldgenEvaluationKey {
            scope: UsfChunkAddress::containing(position, scale)?,
            temporal_scale,
            epoch: epoch.id(),
        })
    }

    /// Ensures one exact canonical branch exists from Scale +35 through
    /// `target_scale`. Existing ancestors/siblings are reused rather than
    /// regenerated or eagerly expanding the full hierarchy.
    pub fn ensure_branch(
        &mut self,
        target: UsfPosition,
        target_scale: SpatialScale,
        temporal_scale: TemporalScale,
        epoch: WorldgenEpoch,
        registry: &PhenomenonRegistry,
    ) -> Result<WorldgenEvaluationKey, UsfPositionError> {
        for raw_scale in (target_scale.exponent()..=SPATIAL_SCALE_MAX).rev() {
            let scale = SpatialScale::new(raw_scale).expect("validated spatial scale range");
            let scope = UsfChunkAddress::containing(target, scale)?;
            let key = WorldgenEvaluationKey {
                scope,
                temporal_scale,
                epoch: epoch.id(),
            };
            if self.nodes.contains_key(&key) {
                continue;
            }

            let parent = scope.parent().map(|parent_scope| WorldgenEvaluationKey {
                scope: parent_scope,
                temporal_scale,
                epoch: epoch.id(),
            });
            let seed = scope_seed(self.universe_seed, scope);
            let context = PhenomenonEvaluationContext { key, epoch, seed };
            let phenomena = {
                let parent_node = parent.and_then(|parent_key| self.nodes.get(&parent_key));
                registry.evaluate(&context, parent_node)
            };
            debug_assert!(
                !phenomena.is_empty(),
                "every atlas scale in the first +35 -> 0 path should be semantically interpreted"
            );

            self.nodes.insert(
                key,
                WorldgenNode {
                    context,
                    parent,
                    phenomena,
                },
            );
        }

        self.key_for(target, target_scale, temporal_scale, epoch)
    }

    pub fn node(&self, key: WorldgenEvaluationKey) -> Option<&WorldgenNode> {
        self.nodes.get(&key)
    }

    pub fn state<T: Any>(&self, key: WorldgenEvaluationKey, id: PhenomenonId) -> Option<&T> {
        self.node(key)?.state::<T>(id)
    }

    /// Returns leaf -> root ancestry for one generated evaluation.
    pub fn lineage(&self, mut key: WorldgenEvaluationKey) -> Vec<&WorldgenNode> {
        let mut lineage = Vec::new();
        while let Some(node) = self.nodes.get(&key) {
            lineage.push(node);
            let Some(parent) = node.parent else {
                break;
            };
            key = parent;
        }
        lineage
    }
}

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhenomenonRegistry>()
            .init_resource::<WorldgenStore>();
    }
}

// --- Built-in first-pass domain states -------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct CosmologicalBackgroundState {
    pub matter_fraction: f32,
    pub baryon_fraction: f32,
    pub dark_energy_fraction: f32,
    pub background_temperature_k: f32,
    pub density_contrast: f32,
    pub tidal_bias: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct CosmicMatterDistributionState {
    pub density_contrast: f32,
    pub filament_strength: f32,
    pub collapse_potential: f32,
    pub void_strength: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct HaloGalaxyEnvironmentState {
    pub halo_mass_bias: f32,
    pub angular_momentum: f32,
    pub baryon_retention: f32,
    pub metallicity: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct GalaxyInterstellarMediumState {
    pub stellar_density: f32,
    pub gas_fraction: f32,
    pub metallicity: f32,
    pub turbulence: f32,
    pub star_formation_potential: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct StellarSystemEnvironmentState {
    pub host_mass_solar: f32,
    pub metallicity: f32,
    pub system_age_gyr: f32,
    pub disk_mass_fraction: f32,
    pub heavy_element_budget: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct PlanetaryBodyState {
    pub body_mass_earth: f32,
    pub radius_earth: f32,
    pub volatile_fraction: f32,
    pub water_inventory: f32,
    pub internal_heat: f32,
    pub insolation: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct GeologyClimateHydrologyState {
    pub terrain_seed: u32,
    pub tectonic_activity: f32,
    pub erosion_strength: f32,
    pub mean_temperature_c: f32,
    pub moisture: f32,
    pub rockiness: f32,
    pub cave_potential: f32,
    pub clay_fraction: f32,
    pub local_relief_m: f32,
    pub terrain_frequency: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct EcologyState {
    pub productivity: f32,
    pub forest_affinity: f32,
    pub grass_affinity: f32,
    pub wetland_affinity: f32,
    pub disturbance: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct MaterialSubstrateState {
    pub clay_fraction: f32,
    pub water_content: f32,
    pub porosity: f32,
    pub compaction: f32,
    pub temperature_c: f32,
    pub fracture: f32,
}

struct CosmologicalBackgroundRule;
struct CosmicMatterDistributionRule;
struct HaloGalaxyEnvironmentRule;
struct GalaxyInterstellarMediumRule;
struct StellarSystemEnvironmentRule;
struct PlanetaryBodyRule;
struct GeologyClimateHydrologyRule;
struct EcologyRule;
struct MaterialSubstrateRule;

impl PhenomenonRule for CosmologicalBackgroundRule {
    fn id(&self) -> PhenomenonId {
        COSMOLOGICAL_BACKGROUND
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        _current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(24..=35).contains(&scale) {
            return None;
        }

        let state = if scale == 35 {
            CosmologicalBackgroundState {
                matter_fraction: 0.315,
                baryon_fraction: 0.049,
                dark_energy_fraction: 0.685,
                background_temperature_k: 2.7255,
                density_contrast: signed_noise(context, 0xC05A_0001) * 1.0e-5,
                tidal_bias: signed_noise(context, 0xC05A_0002) * 1.0e-5,
            }
        } else {
            let parent = parent_state::<CosmologicalBackgroundState>(
                parent,
                COSMOLOGICAL_BACKGROUND,
            )?;
            let depth = (35 - scale) as i32;
            let band_amplitude = 1.0e-5 * 1.55_f32.powi(depth);
            CosmologicalBackgroundState {
                matter_fraction: parent.matter_fraction,
                baryon_fraction: parent.baryon_fraction,
                dark_energy_fraction: parent.dark_energy_fraction,
                background_temperature_k: parent.background_temperature_k,
                density_contrast: parent.density_contrast
                    + signed_noise(context, 0xC05A_1001) * band_amplitude,
                tidal_bias: parent.tidal_bias
                    + signed_noise(context, 0xC05A_1002) * band_amplitude * 0.7,
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "δ={:.3e}, tidal={:.3e}, Tcmb={:.4} K",
                state.density_contrast, state.tidal_bias, state.background_temperature_k
            ),
        ))
    }
}

impl PhenomenonRule for CosmicMatterDistributionRule {
    fn id(&self) -> PhenomenonId {
        COSMIC_MATTER_DISTRIBUTION
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(20..=24).contains(&scale) {
            return None;
        }

        let state = if scale == 24 {
            let cosmology = current_state::<CosmologicalBackgroundState>(
                current,
                COSMOLOGICAL_BACKGROUND,
            )?;
            let contrast = cosmology.density_contrast * 80.0;
            CosmicMatterDistributionState {
                density_contrast: contrast,
                filament_strength: (0.30 + contrast.abs() * 4.0
                    + signed_noise(context, 0xC05C_0001) * 0.08)
                    .clamp(0.0, 1.0),
                collapse_potential: (0.45 + contrast * 3.0
                    + signed_noise(context, 0xC05C_0002) * 0.12)
                    .clamp(0.0, 1.0),
                void_strength: (0.45 - contrast * 2.0
                    + signed_noise(context, 0xC05C_0003) * 0.12)
                    .clamp(0.0, 1.0),
            }
        } else {
            let parent = parent_state::<CosmicMatterDistributionState>(
                parent,
                COSMIC_MATTER_DISTRIBUTION,
            )?;
            CosmicMatterDistributionState {
                density_contrast: parent.density_contrast
                    + signed_noise(context, 0xC05C_1001) * 0.08,
                filament_strength: (parent.filament_strength
                    + 0.08
                    + signed_noise(context, 0xC05C_1002) * 0.07)
                    .clamp(0.0, 1.0),
                collapse_potential: (parent.collapse_potential
                    + signed_noise(context, 0xC05C_1003) * 0.11)
                    .clamp(0.0, 1.0),
                void_strength: (parent.void_strength
                    + signed_noise(context, 0xC05C_1004) * 0.09)
                    .clamp(0.0, 1.0),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "web={:.2}, collapse={:.2}, void={:.2}",
                state.filament_strength, state.collapse_potential, state.void_strength
            ),
        ))
    }
}

impl PhenomenonRule for HaloGalaxyEnvironmentRule {
    fn id(&self) -> PhenomenonId {
        HALO_GALAXY_ENVIRONMENT
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(18..=20).contains(&scale) {
            return None;
        }

        let state = if scale == 20 {
            let cosmic = current_state::<CosmicMatterDistributionState>(
                current,
                COSMIC_MATTER_DISTRIBUTION,
            )?;
            HaloGalaxyEnvironmentState {
                halo_mass_bias: (0.35
                    + cosmic.collapse_potential * 0.55
                    + signed_noise(context, 0xA110_0001) * 0.10)
                    .clamp(0.0, 1.0),
                angular_momentum: unit_noise(context, 0xA110_0002),
                baryon_retention: (0.45
                    + cosmic.filament_strength * 0.4
                    + signed_noise(context, 0xA110_0003) * 0.08)
                    .clamp(0.1, 1.0),
                metallicity: (0.006 + cosmic.collapse_potential * 0.010).clamp(0.001, 0.03),
            }
        } else {
            let parent = parent_state::<HaloGalaxyEnvironmentState>(
                parent,
                HALO_GALAXY_ENVIRONMENT,
            )?;
            HaloGalaxyEnvironmentState {
                halo_mass_bias: (parent.halo_mass_bias
                    + signed_noise(context, 0xA110_1001) * 0.12)
                    .clamp(0.0, 1.0),
                angular_momentum: (parent.angular_momentum * 0.8
                    + unit_noise(context, 0xA110_1002) * 0.2)
                    .clamp(0.0, 1.0),
                baryon_retention: (parent.baryon_retention
                    + signed_noise(context, 0xA110_1003) * 0.06)
                    .clamp(0.1, 1.0),
                metallicity: (parent.metallicity
                    + signed_noise(context, 0xA110_1004) * 0.0015)
                    .clamp(0.001, 0.04),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "halo={:.2}, spin={:.2}, Z={:.3}",
                state.halo_mass_bias, state.angular_momentum, state.metallicity
            ),
        ))
    }
}

impl PhenomenonRule for GalaxyInterstellarMediumRule {
    fn id(&self) -> PhenomenonId {
        GALAXY_INTERSTELLAR_MEDIUM
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(14..=18).contains(&scale) {
            return None;
        }

        let state = if scale == 18 {
            let halo = current_state::<HaloGalaxyEnvironmentState>(
                current,
                HALO_GALAXY_ENVIRONMENT,
            )?;
            GalaxyInterstellarMediumState {
                stellar_density: (0.25 + halo.halo_mass_bias * 0.65).clamp(0.0, 1.0),
                gas_fraction: (0.50 + halo.baryon_retention * 0.25
                    + signed_noise(context, 0x6A1A_0001) * 0.10)
                    .clamp(0.05, 0.9),
                metallicity: halo.metallicity,
                turbulence: (0.35 + halo.angular_momentum * 0.35
                    + signed_noise(context, 0x6A1A_0002) * 0.12)
                    .clamp(0.0, 1.0),
                star_formation_potential: (0.30
                    + halo.baryon_retention * 0.45
                    + signed_noise(context, 0x6A1A_0003) * 0.15)
                    .clamp(0.0, 1.0),
            }
        } else {
            let parent = parent_state::<GalaxyInterstellarMediumState>(
                parent,
                GALAXY_INTERSTELLAR_MEDIUM,
            )?;
            GalaxyInterstellarMediumState {
                stellar_density: (parent.stellar_density
                    + signed_noise(context, 0x6A1A_1001) * 0.13)
                    .clamp(0.0, 1.0),
                gas_fraction: (parent.gas_fraction
                    + signed_noise(context, 0x6A1A_1002) * 0.10)
                    .clamp(0.02, 0.95),
                metallicity: (parent.metallicity
                    + signed_noise(context, 0x6A1A_1003) * 0.001)
                    .clamp(0.001, 0.04),
                turbulence: (parent.turbulence
                    + signed_noise(context, 0x6A1A_1004) * 0.12)
                    .clamp(0.0, 1.0),
                star_formation_potential: (parent.star_formation_potential
                    + signed_noise(context, 0x6A1A_1005) * 0.13)
                    .clamp(0.0, 1.0),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "stars={:.2}, gas={:.2}, SF={:.2}, Z={:.3}",
                state.stellar_density,
                state.gas_fraction,
                state.star_formation_potential,
                state.metallicity
            ),
        ))
    }
}

impl PhenomenonRule for StellarSystemEnvironmentRule {
    fn id(&self) -> PhenomenonId {
        STELLAR_SYSTEM_ENVIRONMENT
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(8..=14).contains(&scale) {
            return None;
        }

        let state = if scale == 14 {
            let galaxy = current_state::<GalaxyInterstellarMediumState>(
                current,
                GALAXY_INTERSTELLAR_MEDIUM,
            )?;
            StellarSystemEnvironmentState {
                // Deliberately Solar-ish bootstrap bias for the first playable branch.
                host_mass_solar: (0.95 + signed_noise(context, 0x57E1_0001) * 0.18)
                    .clamp(0.55, 1.45),
                metallicity: (galaxy.metallicity * 0.55 + 0.007)
                    .clamp(0.004, 0.03),
                system_age_gyr: (4.6 + signed_noise(context, 0x57E1_0002) * 1.3)
                    .clamp(1.0, 10.0),
                disk_mass_fraction: (0.025
                    + galaxy.gas_fraction * 0.035
                    + signed_noise(context, 0x57E1_0003) * 0.01)
                    .clamp(0.005, 0.12),
                heavy_element_budget: (galaxy.metallicity / 0.02).clamp(0.1, 2.0),
            }
        } else {
            let parent = parent_state::<StellarSystemEnvironmentState>(
                parent,
                STELLAR_SYSTEM_ENVIRONMENT,
            )?;
            StellarSystemEnvironmentState {
                host_mass_solar: parent.host_mass_solar,
                metallicity: parent.metallicity,
                system_age_gyr: parent.system_age_gyr,
                disk_mass_fraction: (parent.disk_mass_fraction
                    + signed_noise(context, 0x57E1_1001) * 0.004)
                    .clamp(0.002, 0.15),
                heavy_element_budget: (parent.heavy_element_budget
                    + signed_noise(context, 0x57E1_1002) * 0.06)
                    .clamp(0.05, 2.5),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "host={:.2} solar masses, age={:.2} Gyr, metals={:.2}",
                state.host_mass_solar, state.system_age_gyr, state.heavy_element_budget
            ),
        ))
    }
}

impl PhenomenonRule for PlanetaryBodyRule {
    fn id(&self) -> PhenomenonId {
        PLANETARY_BODY
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(4..=8).contains(&scale) {
            return None;
        }

        let state = if scale == 8 {
            let system = current_state::<StellarSystemEnvironmentState>(
                current,
                STELLAR_SYSTEM_ENVIRONMENT,
            )?;
            PlanetaryBodyState {
                // First branch intentionally biases toward an Earth-like playable body.
                body_mass_earth: (1.0 + signed_noise(context, 0xB0D1_0001) * 0.22)
                    .clamp(0.55, 1.8),
                radius_earth: (1.0 + signed_noise(context, 0xB0D1_0002) * 0.11)
                    .clamp(0.7, 1.35),
                volatile_fraction: (0.35
                    + system.heavy_element_budget * 0.12
                    + signed_noise(context, 0xB0D1_0003) * 0.10)
                    .clamp(0.05, 0.8),
                water_inventory: (0.55 + signed_noise(context, 0xB0D1_0004) * 0.20)
                    .clamp(0.05, 0.95),
                internal_heat: (0.55 + signed_noise(context, 0xB0D1_0005) * 0.15)
                    .clamp(0.1, 1.0),
                insolation: (1.0 + signed_noise(context, 0xB0D1_0006) * 0.10)
                    .clamp(0.75, 1.25),
            }
        } else {
            let parent = parent_state::<PlanetaryBodyState>(parent, PLANETARY_BODY)?;
            PlanetaryBodyState {
                body_mass_earth: parent.body_mass_earth,
                radius_earth: parent.radius_earth,
                volatile_fraction: (parent.volatile_fraction
                    + signed_noise(context, 0xB0D1_1001) * 0.025)
                    .clamp(0.02, 0.9),
                water_inventory: (parent.water_inventory
                    + signed_noise(context, 0xB0D1_1002) * 0.035)
                    .clamp(0.0, 1.0),
                internal_heat: (parent.internal_heat
                    + signed_noise(context, 0xB0D1_1003) * 0.025)
                    .clamp(0.0, 1.0),
                insolation: (parent.insolation
                    + signed_noise(context, 0xB0D1_1004) * 0.02)
                    .clamp(0.2, 2.0),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "mass={:.2} Earth, radius={:.2} Earth, water={:.2}, heat={:.2}",
                state.body_mass_earth,
                state.radius_earth,
                state.water_inventory,
                state.internal_heat
            ),
        ))
    }
}

impl PhenomenonRule for GeologyClimateHydrologyRule {
    fn id(&self) -> PhenomenonId {
        GEOLOGY_CLIMATE_HYDROLOGY
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(0..=4).contains(&scale) {
            return None;
        }

        let state = if scale == 4 {
            let body = current_state::<PlanetaryBodyState>(current, PLANETARY_BODY)?;
            let tectonic = (0.35
                + body.internal_heat * 0.5
                + signed_noise(context, 0x6E01_0001) * 0.10)
                .clamp(0.0, 1.0);
            let moisture = (0.20
                + body.water_inventory * 0.65
                + signed_noise(context, 0x6E01_0002) * 0.12)
                .clamp(0.0, 1.0);
            GeologyClimateHydrologyState {
                terrain_seed: context.seed() as u32,
                tectonic_activity: tectonic,
                erosion_strength: (0.25 + moisture * 0.45).clamp(0.0, 1.0),
                mean_temperature_c: 14.0
                    + (body.insolation - 1.0) * 22.0
                    + signed_noise(context, 0x6E01_0003) * 4.0,
                moisture,
                rockiness: (0.35 + tectonic * 0.4
                    + signed_noise(context, 0x6E01_0004) * 0.12)
                    .clamp(0.0, 1.0),
                cave_potential: (0.25 + moisture * 0.25 + tectonic * 0.15).clamp(0.0, 1.0),
                clay_fraction: (0.20 + moisture * 0.35
                    + signed_noise(context, 0x6E01_0005) * 0.10)
                    .clamp(0.0, 1.0),
                local_relief_m: 6.0 + tectonic * 6.0,
                terrain_frequency: 0.025,
            }
        } else {
            let parent = parent_state::<GeologyClimateHydrologyState>(
                parent,
                GEOLOGY_CLIMATE_HYDROLOGY,
            )?;
            GeologyClimateHydrologyState {
                terrain_seed: context.seed() as u32,
                tectonic_activity: (parent.tectonic_activity
                    + signed_noise(context, 0x6E01_1001) * 0.05)
                    .clamp(0.0, 1.0),
                erosion_strength: (parent.erosion_strength
                    + signed_noise(context, 0x6E01_1002) * 0.06)
                    .clamp(0.0, 1.0),
                mean_temperature_c: parent.mean_temperature_c
                    + signed_noise(context, 0x6E01_1003) * 1.5,
                moisture: (parent.moisture
                    + signed_noise(context, 0x6E01_1004) * 0.08)
                    .clamp(0.0, 1.0),
                rockiness: (parent.rockiness
                    + signed_noise(context, 0x6E01_1005) * 0.08)
                    .clamp(0.0, 1.0),
                cave_potential: (parent.cave_potential
                    + signed_noise(context, 0x6E01_1006) * 0.08)
                    .clamp(0.0, 1.0),
                clay_fraction: (parent.clay_fraction
                    + signed_noise(context, 0x6E01_1007) * 0.08)
                    .clamp(0.0, 1.0),
                local_relief_m: (parent.local_relief_m * 0.88
                    + 2.0
                    + signed_noise(context, 0x6E01_1008) * 1.5)
                    .clamp(2.0, 18.0),
                terrain_frequency: (parent.terrain_frequency * 1.17).clamp(0.018, 0.065),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "T={:.1} C, water={:.2}, relief={:.1}m, caves={:.2}, clay={:.2}",
                state.mean_temperature_c,
                state.moisture,
                state.local_relief_m,
                state.cave_potential,
                state.clay_fraction
            ),
        ))
    }
}

impl PhenomenonRule for EcologyRule {
    fn id(&self) -> PhenomenonId {
        ECOLOGY
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(0..=2).contains(&scale) {
            return None;
        }

        let geology = current_state::<GeologyClimateHydrologyState>(
            current,
            GEOLOGY_CLIMATE_HYDROLOGY,
        )?;
        let temperature_fit = (1.0 - ((geology.mean_temperature_c - 16.0) / 28.0).abs())
            .clamp(0.0, 1.0);
        let base_productivity = geology.moisture * temperature_fit;
        let parent_ecology = parent_state::<EcologyState>(parent, ECOLOGY);
        let inherited = parent_ecology.map_or(base_productivity, |state| state.productivity);
        let productivity = (inherited * 0.55
            + base_productivity * 0.45
            + signed_noise(context, 0xEC01_0001) * 0.08)
            .clamp(0.0, 1.0);
        let disturbance = (0.18
            + geology.tectonic_activity * 0.12
            + signed_noise(context, 0xEC01_0002) * 0.12)
            .clamp(0.0, 1.0);
        let state = EcologyState {
            productivity,
            forest_affinity: (productivity * 0.75
                + geology.moisture * 0.25
                - disturbance * 0.20)
                .clamp(0.0, 1.0),
            grass_affinity: (0.45 + productivity * 0.30
                - geology.moisture * 0.12
                + disturbance * 0.12)
                .clamp(0.0, 1.0),
            wetland_affinity: (geology.moisture * 0.80
                + (1.0 - geology.rockiness) * 0.20)
                .clamp(0.0, 1.0),
            disturbance,
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "productivity={:.2}, forest={:.2}, grass={:.2}, wetland={:.2}",
                state.productivity,
                state.forest_affinity,
                state.grass_affinity,
                state.wetland_affinity
            ),
        ))
    }
}

impl PhenomenonRule for MaterialSubstrateRule {
    fn id(&self) -> PhenomenonId {
        MATERIAL_SUBSTRATE
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        _parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(0..=1).contains(&scale) {
            return None;
        }

        let geology = current_state::<GeologyClimateHydrologyState>(
            current,
            GEOLOGY_CLIMATE_HYDROLOGY,
        )?;
        let state = MaterialSubstrateState {
            clay_fraction: geology.clay_fraction,
            water_content: (geology.moisture * 0.72
                + signed_noise(context, 0xAA71_0001) * 0.08)
                .clamp(0.0, 1.0),
            porosity: (0.28 + geology.clay_fraction * 0.22
                + signed_noise(context, 0xAA71_0002) * 0.06)
                .clamp(0.08, 0.75),
            compaction: (0.62 + geology.rockiness * 0.18
                + signed_noise(context, 0xAA71_0003) * 0.08)
                .clamp(0.0, 1.0),
            temperature_c: geology.mean_temperature_c,
            fracture: (0.18 + geology.tectonic_activity * 0.35
                + signed_noise(context, 0xAA71_0004) * 0.10)
                .clamp(0.0, 1.0),
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "clay={:.2}, water={:.2}, porosity={:.2}, compact={:.2}",
                state.clay_fraction, state.water_content, state.porosity, state.compaction
            ),
        ))
    }
}

fn current_state<'a, T: Any>(
    current: &'a [PhenomenonSnapshot],
    id: PhenomenonId,
) -> Option<&'a T> {
    let snapshot = current.iter().find(|snapshot| snapshot.id() == id)?;
    snapshot.state::<T>()
}

fn parent_state<'a, T: Any>(
    parent: Option<&'a WorldgenNode>,
    id: PhenomenonId,
) -> Option<&'a T> {
    parent?.state::<T>(id)
}

fn scope_seed(universe_seed: u64, scope: UsfChunkAddress) -> u64 {
    let mut state = mix64(universe_seed ^ (scope.scale().exponent() as i64 as u64));
    for raw_scale in (scope.scale().exponent()..=SPATIAL_SCALE_MAX).rev() {
        let scale = SpatialScale::new(raw_scale).expect("validated spatial scale range");
        let digit = scope
            .digit(scale)
            .expect("address stores every digit at and above its scale");
        state = mix64(state ^ (digit.x as i64 as u64).wrapping_mul(0x9E37_79B9));
        state = mix64(state ^ (digit.y as i64 as u64).wrapping_mul(0x85EB_CA6B));
        state = mix64(state ^ (digit.z as i64 as u64).wrapping_mul(0xC2B2_AE35));
    }
    state
}

fn unit_noise(context: &PhenomenonEvaluationContext, salt: u64) -> f32 {
    let value = mix64(context.seed() ^ salt);
    let unit = (value >> 11) as f64 / ((1_u64 << 53) - 1) as f64;
    unit as f32
}

fn signed_noise(context: &PhenomenonEvaluationContext, salt: u64) -> f32 {
    unit_noise(context, salt) * 2.0 - 1.0
}

fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_scale_zero_request_builds_every_nonnegative_scale() {
        let registry = PhenomenonRegistry::default();
        let mut store = WorldgenStore::new(7);
        let epoch = WorldgenEpoch::present_day_bootstrap();
        let leaf = store
            .ensure_branch(
                UsfPosition::default(),
                SpatialScale::ZERO,
                TemporalScale::WORLDGEN_SNAPSHOT,
                epoch,
                &registry,
            )
            .unwrap();

        let lineage = store.lineage(leaf);
        assert_eq!(lineage.len(), 36);
        assert_eq!(lineage.first().unwrap().context().spatial_scale(), SpatialScale::ZERO);
        assert_eq!(lineage.last().unwrap().context().spatial_scale(), SpatialScale::MAX);
        assert!(lineage.iter().all(|node| node.phenomena().len() > 0));
        assert!(store
            .state::<GeologyClimateHydrologyState>(leaf, GEOLOGY_CLIMATE_HYDROLOGY)
            .is_some());
        assert!(store
            .state::<EcologyState>(leaf, ECOLOGY)
            .is_some());
        assert!(store
            .state::<MaterialSubstrateState>(leaf, MATERIAL_SUBSTRATE)
            .is_some());
    }

    #[test]
    fn nearby_scale_zero_branch_reuses_all_shared_ancestors() {
        let registry = PhenomenonRegistry::default();
        let mut store = WorldgenStore::new(11);
        let epoch = WorldgenEpoch::present_day_bootstrap();
        let temporal = TemporalScale::WORLDGEN_SNAPSHOT;
        store
            .ensure_branch(
                UsfPosition::default(),
                SpatialScale::ZERO,
                temporal,
                epoch,
                &registry,
            )
            .unwrap();
        assert_eq!(store.len(), 36);

        let neighbor = UsfPosition::default()
            .translated_native(Vec3::new(1_500.0, 0.0, 0.0))
            .unwrap();
        store
            .ensure_branch(neighbor, SpatialScale::ZERO, temporal, epoch, &registry)
            .unwrap();

        // 1.5 km changes the S0 chunk while remaining in the same S+1 parent.
        assert_eq!(store.len(), 37);
    }

    #[test]
    fn temporal_parameterizations_have_distinct_cache_identity() {
        let registry = PhenomenonRegistry::default();
        let mut store = WorldgenStore::new(13);
        let epoch = WorldgenEpoch::present_day_bootstrap();
        let target = UsfPosition::default();
        store
            .ensure_branch(
                target,
                SpatialScale::ZERO,
                TemporalScale::WORLDGEN_SNAPSHOT,
                epoch,
                &registry,
            )
            .unwrap();
        store
            .ensure_branch(
                target,
                SpatialScale::ZERO,
                TemporalScale::new(1),
                epoch,
                &registry,
            )
            .unwrap();

        assert_eq!(store.len(), 72);
    }
}
