//! Voxel-local presentation-resolution contracts.
//!
//! USF [`SpatialScale`] answers "which bounded numerical chart should this
//! backend use?". It must not also answer "how densely should this semantic
//! field be polygonized?". Those are independent axes.
//!
//! This module is intentionally a shadow mechanism today. Existing dense
//! [`super::VoxelWorld`] materializations remain the live editable/collision
//! working caches. Presentation will migrate onto this resolution domain only
//! after the balancing + transition-meshing contract is proven.

// voxel-presentation-resolution-domain-v1

#![allow(dead_code)]

mod gpu;
mod live;

pub(super) use live::{
    CelestialClipmapCoverageCell, CelestialClipmapCoverageSnapshot,
    CelestialClipmapTelemetry,
};

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
    pub(crate) fn request(
        &mut self,
        block: IVec3,
        resolution: VoxelPresentationResolution,
    ) {
        self.levels
            .entry(block)
            .and_modify(|current| {
                if resolution < *current {
                    *current = resolution;
                }
            })
            .or_insert(resolution);
    }

    pub(crate) fn resolution(
        &self,
        block: IVec3,
    ) -> Option<VoxelPresentationResolution> {
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
            let mut corrections =
                HashMap::<IVec3, VoxelPresentationResolution>::new();

            for (&block, &resolution) in &self.levels {
                for (delta, _) in NEIGHBORS {
                    let Some(&neighbor) = self.levels.get(&(block + delta)) else {
                        continue;
                    };

                    let difference =
                        i32::from(resolution.binary_exponent())
                            - i32::from(neighbor.binary_exponent());
                    if difference <= 1 {
                        continue;
                    }

                    let Some(target_exponent) =
                        neighbor.binary_exponent().checked_add(1)
                    else {
                        continue;
                    };
                    let target =
                        VoxelPresentationResolution::new(target_exponent);
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
    pub(crate) fn transition_faces(
        &self,
        block: IVec3,
    ) -> VoxelTransitionFaces {
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

#[cfg(test)]
mod transvoxel_proof;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_resolution_is_independent_from_usf_chart() {
        let resolution = VoxelPresentationResolution::new(10);
        let spacing_metres = resolution.sample_spacing_metres();
        assert_eq!(spacing_metres, 1024.0);

        for chart in [
            SpatialScale::new(3).unwrap(),
            SpatialScale::new(5).unwrap(),
        ] {
            let native = resolution.sample_spacing_native(chart).unwrap();
            let reconstructed = native * chart.metres_per_native();
            assert!((reconstructed - spacing_metres).abs() < 1.0e-9);
        }
    }

    #[test]
    fn requested_spacing_selects_binary_ladder_without_usf_scale() {
        assert_eq!(
            VoxelPresentationResolution::at_most_metres(10.0)
                .unwrap()
                .sample_spacing_metres(),
            8.0,
        );
        assert_eq!(
            VoxelPresentationResolution::at_most_metres(100.0)
                .unwrap()
                .sample_spacing_metres(),
            64.0,
        );
    }

    #[test]
    fn balancing_refines_a_too_coarse_neighbor_to_two_to_one() {
        let mut plan = VoxelPresentationResolutionPlan::default();
        plan.request(
            IVec3::ZERO,
            VoxelPresentationResolution::new(4),
        );
        plan.request(
            IVec3::X,
            VoxelPresentationResolution::new(0),
        );

        assert!(!plan.is_balanced_2_to_1());
        plan.balance_2_to_1();

        assert!(plan.is_balanced_2_to_1());
        assert_eq!(
            plan.resolution(IVec3::ZERO),
            Some(VoxelPresentationResolution::new(1)),
        );
        assert_eq!(
            plan.resolution(IVec3::X),
            Some(VoxelPresentationResolution::new(0)),
        );
    }

    #[test]
    fn transition_face_is_owned_by_coarse_block_toward_fine_neighbor() {
        let mut plan = VoxelPresentationResolutionPlan::default();
        plan.request(
            IVec3::ZERO,
            VoxelPresentationResolution::new(1),
        );
        plan.request(
            IVec3::X,
            VoxelPresentationResolution::new(0),
        );
        plan.balance_2_to_1();

        let coarse = plan.transition_faces(IVec3::ZERO);
        let fine = plan.transition_faces(IVec3::X);

        assert!(coarse.contains(VoxelTransitionFace::HighX));
        assert!(fine.is_empty());
    }

    #[test]
    fn finest_duplicate_request_wins_without_new_authority() {
        let mut plan = VoxelPresentationResolutionPlan::default();
        plan.request(
            IVec3::ZERO,
            VoxelPresentationResolution::new(5),
        );
        plan.request(
            IVec3::ZERO,
            VoxelPresentationResolution::new(2),
        );
        plan.request(
            IVec3::ZERO,
            VoxelPresentationResolution::new(4),
        );

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan.resolution(IVec3::ZERO),
            Some(VoxelPresentationResolution::new(2)),
        );
    }
}


pub(super) fn configure(app: &mut bevy::prelude::App) {
    gpu::configure(app);
    live::configure(app);
}
