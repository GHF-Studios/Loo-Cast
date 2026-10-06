//! Voxel-local presentation-resolution contracts.
//!
//! USF [`SpatialScale`] answers "which bounded numerical chart should this
//! backend use?". It must not also answer "how densely should this semantic
//! field be polygonized?". Those are independent axes.
//!
//! The binary clipmap is live presentation. Dense [`super::VoxelScaleRealization`]
//! materializations remain independent editable/collision working caches.
//!
//! ## Module map
//!
//! - `classification`: Field-owned shell exclusion and sparse boundary evidence for clipmap
//!   planning.
//! - `gpu`: GPU build backend for binary terrain presentation.
//! - `live`: Live celestial presentation facility over the voxel-local binary resolution domain.
//! - `topology`: Pure dyadic topology for body-local binary presentation blocks.
//! - `visibility`: Body-local camera demand for the reconstructible clipmap frontier.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

#![allow(dead_code)]

mod classification;
mod gpu;
mod live;
mod topology;
mod visibility;

pub(super) use live::CelestialClipmapTelemetry;

use std::collections::HashMap;

use bevy::prelude::IVec3;

use crate::spatial::SpatialScale;

/// Binary physical sampling level for voxel presentation.
///
/// Level 0 is one metre between scalar samples. Positive levels are coarser by
/// powers of two; negative levels are finer. The exponent is deliberately not a
/// USF decimal Scale exponent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct VoxelPresentationResolution(i16);

impl VoxelPresentationResolution {
    pub(crate) const METRE: Self = Self(0);

    pub(crate) const fn new(binary_exponent: i16) -> Self {
        Self(binary_exponent)
    }

    pub(crate) const fn binary_exponent(self) -> i16 {
        self.0
    }

    pub(crate) fn sample_spacing_metres(self) -> f64 {
        2.0_f64.powi(i32::from(self.0))
    }

    /// Chooses the finest binary spacing that does not exceed the requested
    /// physical spacing.
    pub(crate) fn at_most_metres(max_spacing_metres: f64) -> Option<Self> {
        if !max_spacing_metres.is_finite() || max_spacing_metres <= 0.0 {
            return None;
        }

        let exponent = max_spacing_metres.log2().floor();
        if exponent < f64::from(i16::MIN) || exponent > f64::from(i16::MAX) {
            return None;
        }
        Some(Self(exponent as i16))
    }

    pub(crate) fn at_least_metres(min_spacing_metres: f64) -> Option<Self> {
        if !min_spacing_metres.is_finite() || min_spacing_metres <= 0.0 {
            return None;
        }
        let exponent = min_spacing_metres.log2().ceil();
        if exponent < f64::from(i16::MIN) || exponent > f64::from(i16::MAX) {
            return None;
        }
        Some(Self(exponent as i16))
    }

    /// Projects this *physical* resolution into one disposable USF chart.
    ///
    /// Changing the chart changes this numeric value but never the physical
    /// sample spacing represented by the resolution level.
    pub(crate) fn sample_spacing_native(self, chart: SpatialScale) -> Option<f64> {
        let native = self.sample_spacing_metres() / chart.metres_per_native();
        (native.is_finite() && native > 0.0).then_some(native)
    }

    pub(crate) const fn finer(self) -> Option<Self> {
        match self.0.checked_sub(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    pub(crate) const fn coarser(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    pub(crate) fn ratio_to(self, other: Self) -> f64 {
        2.0_f64.powi(i32::from(self.0) - i32::from(other.0))
    }

    pub(crate) fn is_balanced_with(self, other: Self) -> bool {
        (i32::from(self.0) - i32::from(other.0)).abs() <= 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum VoxelTransitionFace {
    LowX,
    HighX,
    LowY,
    HighY,
    LowZ,
    HighZ,
}

impl VoxelTransitionFace {
    const fn bit(self) -> u8 {
        match self {
            Self::LowX => 1 << 0,
            Self::HighX => 1 << 1,
            Self::LowY => 1 << 2,
            Self::HighY => 1 << 3,
            Self::LowZ => 1 << 4,
            Self::HighZ => 1 << 5,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct VoxelTransitionFaces(u8);

impl VoxelTransitionFaces {
    pub(crate) const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub(crate) const fn bits(self) -> u8 {
        self.0
    }

    pub(crate) const fn contains(self, face: VoxelTransitionFace) -> bool {
        self.0 & face.bit() != 0
    }

    pub(crate) fn insert(&mut self, face: VoxelTransitionFace) {
        self.0 |= face.bit();
    }
}

const NEIGHBORS: [(IVec3, VoxelTransitionFace); 6] = [
    (IVec3::new(-1, 0, 0), VoxelTransitionFace::LowX),
    (IVec3::new(1, 0, 0), VoxelTransitionFace::HighX),
    (IVec3::new(0, -1, 0), VoxelTransitionFace::LowY),
    (IVec3::new(0, 1, 0), VoxelTransitionFace::HighY),
    (IVec3::new(0, 0, -1), VoxelTransitionFace::LowZ),
    (IVec3::new(0, 0, 1), VoxelTransitionFace::HighZ),
];

/// Sparse requested resolution over one presentation-local block lattice.
///
/// Coordinates are intentionally local to the eventual presentation plan; they
/// are not universe coordinates and do not create another canonical hierarchy.
#[derive(Debug, Default, Clone)]
pub(crate) struct VoxelPresentationResolutionPlan {
    levels: HashMap<IVec3, VoxelPresentationResolution>,
}

impl VoxelPresentationResolutionPlan {
    pub(crate) fn request(&mut self, block: IVec3, resolution: VoxelPresentationResolution) {
        self.levels
            .entry(block)
            .and_modify(|current| {
                if resolution < *current {
                    *current = resolution;
                }
            })
            .or_insert(resolution);
    }

    pub(crate) fn resolution(&self, block: IVec3) -> Option<VoxelPresentationResolution> {
        self.levels.get(&block).copied()
    }

    pub(crate) fn len(&self) -> usize {
        self.levels.len()
    }

    /// Refines coarse requested blocks until every present face-neighbor is
    /// either equal resolution or exactly 2:1.
    ///
    /// This only refines; it never silently coarsens a caller's request.
    pub(crate) fn balance_2_to_1(&mut self) {
        loop {
            let mut corrections = HashMap::<IVec3, VoxelPresentationResolution>::new();

            for (&block, &resolution) in &self.levels {
                for (delta, _) in NEIGHBORS {
                    let Some(&neighbor) = self.levels.get(&(block + delta)) else {
                        continue;
                    };

                    let difference = i32::from(resolution.binary_exponent())
                        - i32::from(neighbor.binary_exponent());
                    if difference <= 1 {
                        continue;
                    }

                    let Some(target_exponent) = neighbor.binary_exponent().checked_add(1) else {
                        continue;
                    };
                    let target = VoxelPresentationResolution::new(target_exponent);
                    corrections
                        .entry(block)
                        .and_modify(|existing| {
                            if target < *existing {
                                *existing = target;
                            }
                        })
                        .or_insert(target);
                }
            }

            if corrections.is_empty() {
                break;
            }

            for (block, target) in corrections {
                if self
                    .levels
                    .get(&block)
                    .is_some_and(|current| target < *current)
                {
                    self.levels.insert(block, target);
                }
            }
        }
    }

    /// Transition faces belong to the *coarser* block facing an exactly
    /// one-level-finer neighbor.
    pub(crate) fn transition_faces(&self, block: IVec3) -> VoxelTransitionFaces {
        let Some(resolution) = self.resolution(block) else {
            return VoxelTransitionFaces::default();
        };

        let mut faces = VoxelTransitionFaces::default();
        for (delta, face) in NEIGHBORS {
            let Some(neighbor) = self.resolution(block + delta) else {
                continue;
            };
            if neighbor
                .binary_exponent()
                .checked_add(1)
                .is_some_and(|coarse| coarse == resolution.binary_exponent())
            {
                faces.insert(face);
            }
        }
        faces
    }

    pub(crate) fn is_balanced_2_to_1(&self) -> bool {
        self.levels.iter().all(|(&block, &resolution)| {
            NEIGHBORS.iter().all(|(delta, _)| {
                self.resolution(block + *delta)
                    .is_none_or(|neighbor| resolution.is_balanced_with(neighbor))
            })
        })
    }
}

pub(super) fn configure(app: &mut bevy::prelude::App) {
    gpu::configure(app);
    live::configure(app);
}
