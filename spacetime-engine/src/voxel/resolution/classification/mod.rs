//! Field-owned shell exclusion and sparse boundary evidence for clipmap planning.
//! View demand and moving focus are applied by the planner, outside this cache.

use std::collections::HashMap;
use bevy::math::{DVec3, Vec3};

use crate::voxel::{CelestialPresentationFieldSampler, CelestialVoxelField};
use super::topology::CelestialClipmapBlockKey;

//
// Planner state is deliberately reusable. The broad shell result is a field
// exclusion proof; fine boundary samples only prioritize refinement. Neither
// a sample miss nor view exclusion certifies empty geometry.
// Keep recent classifications so movement pays mostly for changed frontier
// work without an unbounded spatial cache.
const MAX_PLANNER_CLASSIFICATIONS: usize = 131_072;
const PLANNER_CLASSIFICATION_RETENTION_GENERATIONS: u64 = 3;

#[derive(Debug, Default, Clone, Copy)]
struct ClipmapSurfaceClassification {
    broad: Option<bool>,
    refinement: Option<RefinementEvidence>,
    touched_generation: u64,
}

/// A sparse probe can prioritize a child, but cannot prove that child empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RefinementEvidence {
    ProvenOutsideShell,
    BroadShell,
    BoundaryObserved,
    NoBoundaryObserved,
    SamplingUnavailable,
}

impl RefinementEvidence {
    fn admits_child(self) -> bool {
        !matches!(self, Self::ProvenOutsideShell | Self::NoBoundaryObserved)
    }
}

#[derive(Default)]
pub(super) struct CelestialClipmapSurfaceCache {
    field: Option<CelestialVoxelField>,
    generation: u64,
    classifications: HashMap<CelestialClipmapBlockKey, ClipmapSurfaceClassification>,
    hits: usize,
    misses: usize,
    cheap_rejects: usize,
}

const MIN_EXACT_PLANNER_BLOCK_EXTENT_METRES: f64 = 262_144.0;
const EXACT_PLANNER_SHELL_MULTIPLIER: f64 = 2.0;

pub(super) struct ClipmapBoundaryClassifier<'a> {
    field: CelestialVoxelField,
    sampler: &'a CelestialPresentationFieldSampler,
    exact_extent_metres: f64,
}

impl<'a> ClipmapBoundaryClassifier<'a> {
    pub(super) fn new(
        field: CelestialVoxelField,
        sampler: &'a CelestialPresentationFieldSampler,
    ) -> Self {
        let (surface_minimum, surface_maximum) =
            field.conservative_surface_radius_bounds_metres();
        let radial_uncertainty = (
            surface_maximum - surface_minimum
                + field.volumetric_surface_inward_support_metres()
        )
            .max(0.0);
        let exact_extent_metres = (
            radial_uncertainty * EXACT_PLANNER_SHELL_MULTIPLIER
        )
            .max(MIN_EXACT_PLANNER_BLOCK_EXTENT_METRES);

        assert!(
            exact_extent_metres.is_finite() && exact_extent_metres > 0.0,
            "planner exact-classification extent must be finite and positive"
        );

        Self {
            field,
            sampler,
            exact_extent_metres,
        }
    }

    fn needs_exact_boundary(
        &self,
        key: CelestialClipmapBlockKey,
    ) -> bool {
        key.extent_metres() <= self.exact_extent_metres
    }

    fn root_intersects(
        &self,
        key: CelestialClipmapBlockKey,
    ) -> bool {
        block_may_intersect_presentation_shell(self.field, key)
    }

    fn refinement_intersects(
        &self,
        key: CelestialClipmapBlockKey,
    ) -> RefinementEvidence {
        if !block_may_intersect_presentation_shell(self.field, key) {
            return RefinementEvidence::ProvenOutsideShell;
        }
        if !self.needs_exact_boundary(key) {
            return RefinementEvidence::BroadShell;
        }

        block_intersects_refinement_boundary_with_sampler(
            self.field,
            key,
            self.sampler,
        )
    }
}

impl CelestialClipmapSurfaceCache {
    pub(super) fn begin_plan(&mut self, field: CelestialVoxelField) {
        self.generation = self.generation.wrapping_add(1).max(1);
        if self.field != Some(field) {
            self.field = Some(field);
            self.classifications.clear();
        }
        self.hits = 0;
        self.misses = 0;
        self.cheap_rejects = 0;
    }


    pub(super) fn intersects(
        &mut self,
        key: CelestialClipmapBlockKey,
        classifier: &ClipmapBoundaryClassifier<'_>,
    ) -> bool {
        if let Some(entry) = self.classifications.get_mut(&key)
            && let Some(value) = entry.broad
        {
            entry.touched_generation = self.generation;
            self.hits = self.hits.saturating_add(1);
            return value;
        }

        self.misses = self.misses.saturating_add(1);
        let value = classifier.root_intersects(key);
        if !value {
            self.cheap_rejects = self.cheap_rejects.saturating_add(1);
        }

        let entry = self.classifications.entry(key).or_default();
        entry.broad = Some(value);
        entry.touched_generation = self.generation;
        value
    }


    pub(super) fn refinement_intersects(
        &mut self,
        key: CelestialClipmapBlockKey,
        classifier: &ClipmapBoundaryClassifier<'_>,
    ) -> bool {
        if let Some(entry) = self.classifications.get_mut(&key)
            && let Some(value) = entry.refinement
        {
            entry.touched_generation = self.generation;
            self.hits = self.hits.saturating_add(1);
            return value.admits_child();
        }

        self.misses = self.misses.saturating_add(1);
        let value = classifier.refinement_intersects(key);
        if value == RefinementEvidence::ProvenOutsideShell {
            self.cheap_rejects = self.cheap_rejects.saturating_add(1);
        }

        let entry = self.classifications.entry(key).or_default();
        entry.refinement = Some(value);
        entry.touched_generation = self.generation;
        value.admits_child()
    }

    pub(super) fn finish_plan(&mut self) {
        if self.classifications.len() <= MAX_PLANNER_CLASSIFICATIONS {
            return;
        }
        let minimum_generation = self
            .generation
            .saturating_sub(PLANNER_CLASSIFICATION_RETENTION_GENERATIONS);
        self.classifications.retain(|_, entry| {
            entry.touched_generation >= minimum_generation
        });
    }
}


fn center_surface_radial_delta(
    key: CelestialClipmapBlockKey,
    sampler: &CelestialPresentationFieldSampler,
) -> Option<f64> {
    let center = key.center_local_metres();
    let radial = center.length();
    if !radial.is_finite() || radial <= f64::EPSILON {
        return None;
    }

    let direction = Vec3::new(
        (center.x / radial) as f32,
        (center.y / radial) as f32,
        (center.z / radial) as f32,
    )
    .normalize_or_zero();
    if direction == Vec3::ZERO {
        return None;
    }

    let surface_radius = sampler.surface_local_metres(direction).ok()?.length();
    if !surface_radius.is_finite() {
        return None;
    }

    debug_assert!(key.extent_metres() > 0.0);
    debug_assert!(center.is_finite());
    Some(radial - surface_radius)
}


/// Fine-refinement occupancy.
///
/// Whole-body roots intentionally keep the broad conservative shell test.
/// Children, however, need evidence of an actual boundary. Treating the whole
/// declared cave inward-support band as occupied at every fine level creates a
/// 3-D volume refinement explosion and starves the visible surface branch.

fn block_intersects_refinement_boundary_with_sampler(
    field: CelestialVoxelField,
    key: CelestialClipmapBlockKey,
    sampler: &CelestialPresentationFieldSampler,
) -> RefinementEvidence {
    if !block_may_intersect_presentation_shell(field, key) {
        return RefinementEvidence::ProvenOutsideShell;
    }

    let radial_threshold =
        key.half_extent_metres().length()
            + key.extent_metres() * 0.20
            + key.spacing_metres() * 2.0;
    if center_surface_radial_delta(key, sampler)
        .is_some_and(|delta| delta.abs() <= radial_threshold)
    {
        return RefinementEvidence::BoundaryObserved;
    }

    let origin = key.origin_local_metres();
    let extent = key.extent_metres();
    let center = key.center_local_metres();
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut minimum_abs = f64::INFINITY;
    let mut samples = 0usize;
    let mut unavailable = false;

    let mut observe = |point: DVec3| {
        if let Some(distance) = sampler.signed_distance_local_metres(point)
            && distance.is_finite()
        {
            minimum = minimum.min(distance);
            maximum = maximum.max(distance);
            minimum_abs = minimum_abs.min(distance.abs());
            samples += 1;
        } else {
            unavailable = true;
        }
    };

    observe(center);
    for z in [0.0_f64, 1.0] {
        for y in [0.0_f64, 1.0] {
            for x in [0.0_f64, 1.0] {
                observe(origin + DVec3::new(
                    x * extent,
                    y * extent,
                    z * extent,
                ));
            }
        }
    }

    if samples == 0 {
        return RefinementEvidence::SamplingUnavailable;
    }
    if minimum <= 0.0 && maximum >= 0.0 {
        return RefinementEvidence::BoundaryObserved;
    }

    let evidence_margin =
        (key.spacing_metres() * 2.0)
            .max(key.half_extent_metres().length() * 0.30);
    if minimum_abs <= evidence_margin {
        RefinementEvidence::BoundaryObserved
    } else if unavailable {
        RefinementEvidence::SamplingUnavailable
    } else {
        RefinementEvidence::NoBoundaryObserved
    }
}


fn block_may_intersect_presentation_shell(
    field: CelestialVoxelField,
    key: CelestialClipmapBlockKey,
) -> bool {
    let minimum = key.origin_local_metres();
    let maximum = minimum + DVec3::splat(key.extent_metres());

    let nearest_axis = |low: f64, high: f64| {
        if low <= 0.0 && high >= 0.0 {
            0.0
        } else {
            low.abs().min(high.abs())
        }
    };
    let farthest_axis =
        |low: f64, high: f64| low.abs().max(high.abs());

    let nearest = DVec3::new(
        nearest_axis(minimum.x, maximum.x),
        nearest_axis(minimum.y, maximum.y),
        nearest_axis(minimum.z, maximum.z),
    )
    .length();
    let farthest = DVec3::new(
        farthest_axis(minimum.x, maximum.x),
        farthest_axis(minimum.y, maximum.y),
        farthest_axis(minimum.z, maximum.z),
    )
    .length();

    let (surface_minimum, surface_maximum) =
        field.conservative_surface_radius_bounds_metres();
    let conservative_extra =
        key.extent_metres() * 0.35 + key.spacing_metres() * 2.0;
    let volumetric_minimum = (
        surface_minimum - field.volumetric_surface_inward_support_metres()
    )
    .max(0.0);

    nearest <= surface_maximum + conservative_extra
        && farthest >= (volumetric_minimum - conservative_extra).max(0.0)
}
