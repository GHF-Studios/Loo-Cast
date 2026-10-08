//! Stable generation provenance and constraint resolution for the USF universe.
//!
//! A world has one persistent root seed. Derived samples are keyed by stable
//! phenomenon paths and field identities, never transient ECS entities, query
//! order, chunk load order, or a mutable global RNG cursor.
//!
//! Authored constraints are evaluated *before* procedural fallback. A fixed
//! fact is not reconstructed from physics or randomized and must survive
//! changes to realization, residency, view or generator sampling order.
//!
//! This module owns the first reusable construction inputs only. Semantic
//! identity/authority and simulation history remain their respective owners;
//! a provenance component is not a second semantic authority or world index.

use bevy::prelude::{Component, Resource};

mod atlas;
pub use atlas::{AnchoredConstruction, ConstructionCatalogError, SparseConstructionAtlas};

/// Persistent world creation input. Serialize/configure it when world saves
/// are introduced; never pick a new random value during each startup.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldSeed(pub u64);

impl Default for WorldSeed {
    fn default() -> Self {
        // Initial fixed Loo-Cast universe seed. This is a world parameter, not
        // a generator-local seed and not a substitute for authored facts.
        Self(0x4c4f_4f43_4153_5421)
    }
}

/// Constraint source of one typed construction fact. Domain-specific
/// constraints (positions, orbits, budgets, etc.) can build on this boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstructionConstraint<T> {
    Procedural,
    Exact(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructionFactSource {
    Procedural,
    AuthoredConstraint,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedConstructionFact<T> {
    value: T,
    source: ConstructionFactSource,
}

impl<T> ResolvedConstructionFact<T> {
    pub fn into_value(self) -> T {
        self.value
    }

    pub const fn source(&self) -> ConstructionFactSource {
        self.source
    }
}

/// Recorded on the *semantic phenomenon*, not its disposable Scale Slice
/// realizations. The stable path survives entity ID and runtime-order changes.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct PhenomenonProvenance {
    key: String,
    generator_revision: u64,
    authored_constraints: bool,
}

impl PhenomenonProvenance {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub const fn generator_revision(&self) -> u64 {
        self.generator_revision
    }

    pub const fn has_authored_constraints(&self) -> bool {
        self.authored_constraints
    }
}

/// Construction context for one stable phenomenon in the canonical universe.
/// It creates no ECS entities, does not load chunks, and owns no simulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhenomenonGeneration {
    world_seed: WorldSeed,
    key: String,
    generator_revision: u64,
}

impl PhenomenonGeneration {
    pub fn new(world_seed: WorldSeed, key: impl Into<String>, generator_revision: u64) -> Self {
        let key = key.into();
        assert!(!key.is_empty(), "phenomenon generation identity must be stable and nonempty");
        Self {
            world_seed,
            key,
            generator_revision,
        }
    }

    pub fn provenance(&self, authored_constraints: bool) -> PhenomenonProvenance {
        PhenomenonProvenance {
            key: self.key.clone(),
            generator_revision: self.generator_revision,
            authored_constraints,
        }
    }

    /// A deterministic domain-separated input for one generator field.
    /// Changing another field or visiting another phenomenon first is inert.
    pub fn sample_seed(&self, field: &str) -> u64 {
        assert!(!field.is_empty(), "generation field must be nonempty");
        let mut hash = fnv_update(FNV_OFFSET, &self.world_seed.0.to_le_bytes());
        hash = fnv_segment(hash, b"usf.phenomenon.v1");
        hash = fnv_segment(hash, self.key.as_bytes());
        hash = fnv_update(hash, &self.generator_revision.to_le_bytes());
        hash = fnv_segment(hash, field.as_bytes());
        splitmix64(hash)
    }

    /// Resolve an authored fact without invoking its procedural fallback.
    /// An unconstrained fact instead receives a stable field-specific sample.
    pub fn resolve<T>(
        &self,
        field: &str,
        constraint: ConstructionConstraint<T>,
        procedural: impl FnOnce(u64) -> T,
    ) -> ResolvedConstructionFact<T> {
        match constraint {
            ConstructionConstraint::Exact(value) => ResolvedConstructionFact {
                value,
                source: ConstructionFactSource::AuthoredConstraint,
            },
            ConstructionConstraint::Procedural => ResolvedConstructionFact {
                value: procedural(self.sample_seed(field)),
                source: ConstructionFactSource::Procedural,
            },
        }
    }
}

/// Stable [0, 1) sample for domain-local scalar generators.
/// No mutable RNG state or global ordering is involved.
pub fn unit_sample(seed: u64) -> f64 {
    ((seed >> 11) as f64) * (1.0 / ((1_u64 << 53) as f64))
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn fnv_update(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state = (state ^ u64::from(*byte)).wrapping_mul(FNV_PRIME);
    }
    state
}

fn fnv_segment(state: u64, bytes: &[u8]) -> u64 {
    fnv_update(fnv_update(state, &(bytes.len() as u64).to_le_bytes()), bytes)
}

fn splitmix64(value: u64) -> u64 {
    let mut value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivation_is_order_independent_and_domain_separated() {
        let seed = WorldSeed::default();
        let earth = PhenomenonGeneration::new(seed, "solar-system/earth", 1);
        let moon = PhenomenonGeneration::new(seed, "solar-system/moon", 1);
        let a = earth.sample_seed("terrain");
        let b = earth.sample_seed("materials");
        assert_eq!(a, earth.sample_seed("terrain"));
        assert_ne!(a, b);
        assert_ne!(a, moon.sample_seed("terrain"));
        assert_ne!(a, PhenomenonGeneration::new(seed, "solar-system/earth", 2).sample_seed("terrain"));
        assert_ne!(a, PhenomenonGeneration::new(WorldSeed(seed.0 ^ 1), "solar-system/earth", 1).sample_seed("terrain"));
        assert!((0.0..1.0).contains(&unit_sample(a)));
    }

    #[test]
    fn exact_constraints_bypass_generation_without_changing_identity() {
        let a = PhenomenonGeneration::new(WorldSeed(1), "solar-system/earth", 1);
        let b = PhenomenonGeneration::new(WorldSeed(2), "solar-system/earth", 1);
        let fixed = |context: &PhenomenonGeneration| {
            context.resolve("radius-metres", ConstructionConstraint::Exact(6_371_000_u64), |_| {
                panic!("fixed real-world fact must never invoke fallback")
            })
        };
        assert_eq!(fixed(&a).into_value(), fixed(&b).into_value());
        assert_eq!(fixed(&a).source(), ConstructionFactSource::AuthoredConstraint);
        assert_eq!(a.provenance(true).key(), b.provenance(true).key());
        let generated = a.resolve("radius-metres", ConstructionConstraint::Procedural, |sample| sample);
        assert_eq!(generated.source(), ConstructionFactSource::Procedural);
        assert_eq!(generated.into_value(), a.sample_seed("radius-metres"));
    }
}
