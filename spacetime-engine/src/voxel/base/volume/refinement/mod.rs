//! Cross-scale USF refinement realization for procedural volumes.

use bevy::prelude::{Vec2, Vec3};

use crate::spatial::{SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SpatialScale};

use super::super::{
    EMPTY_DISTANCE,
    noise::{value_noise, value_noise_3d},
    terrain::ProceduralTerrain,
};
use super::super::super::{VoxelMaterialId, VoxelSample};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ScaleRefinementHierarchy {
    pub(super) current_scale: SpatialScale,
    pub(super) seeds: [u32; SPATIAL_SCALE_COUNT],
}

pub(super) fn sample_refinement_hierarchy(local: Vec3, hierarchy: ScaleRefinementHierarchy) -> VoxelSample {
    let current = hierarchy.current_scale;
    let mut distance = 0.0_f64;

    for raw in (current.exponent()..=SPATIAL_SCALE_MAX).rev() {
        let level = SpatialScale::new(raw).expect("validated refinement scale");
        let exponent_delta = i32::from(level.exponent() - current.exponent());
        let factor = 10.0_f64.powi(exponent_delta);
        let level_local = Vec3::new(
            (local.x as f64 / factor) as f32,
            (local.y as f64 / factor) as f32,
            (local.z as f64 / factor) as f32,
        );
        let seed = hierarchy.seeds[level.index_from_top()];

        if level == SpatialScale::MAX {
            distance = f64::from(root_scale_distance(level_local, seed)) * factor;
        } else {
            distance += f64::from(refinement_detail_offset(level_local, seed)) * factor;
        }
    }

    let distance = distance.clamp(-(EMPTY_DISTANCE as f64), EMPTY_DISTANCE as f64) as f32;
    VoxelSample::new(
        distance,
        if distance < 0.0 {
            VoxelMaterialId::ROCK
        } else {
            VoxelMaterialId::VOID
        },
    )
}

pub(super) fn refinement_surface_height(local: Vec2, hierarchy: ScaleRefinementHierarchy) -> f32 {
    let current = hierarchy.current_scale;
    let mut height = 0.0_f64;

    for raw in (current.exponent()..=SPATIAL_SCALE_MAX).rev() {
        let level = SpatialScale::new(raw).expect("validated refinement scale");
        let exponent_delta = i32::from(level.exponent() - current.exponent());
        let factor = 10.0_f64.powi(exponent_delta);
        let level_local = Vec2::new(
            (local.x as f64 / factor) as f32,
            (local.y as f64 / factor) as f32,
        );
        let seed = hierarchy.seeds[level.index_from_top()];

        let contribution = if level == SpatialScale::MAX {
            root_surface_height(level_local, seed)
        } else {
            refinement_height_detail(level_local, seed)
        };
        height += f64::from(contribution) * factor;
    }

    height.clamp(-(EMPTY_DISTANCE as f64), EMPTY_DISTANCE as f64) as f32
}

fn root_surface_height(local: Vec2, seed: u32) -> f32 {
    ProceduralTerrain::configured(seed, -4.0, 3.0, 0.035).height(local.x, local.y)
}

fn root_scale_distance(local: Vec3, seed: u32) -> f32 {
    let surface_height = root_surface_height(Vec2::new(local.x, local.z), seed);
    let structure = value_noise_3d(local * (0.035 * 0.72), seed ^ 0x31D0_6A5B);
    let exterior = local.y - surface_height + structure * 1.45;

    let cave_noise = value_noise_3d(local * 0.072, seed ^ 0xCA7E_5EED);
    let depth = (surface_height - local.y).max(0.0);
    let underground_gate = (depth / 3.0).clamp(0.0, 1.0);
    let cave = (0.10 - cave_noise.abs()) * 5.0 * underground_gate;
    exterior.max(cave)
}

fn refinement_height_detail(local: Vec2, seed: u32) -> f32 {
    value_noise(local * 0.10, seed ^ 0xB16B_00B5) * 0.32
}

fn refinement_detail_offset(local: Vec3, seed: u32) -> f32 {
    let volumetric = value_noise_3d(local * 0.11, seed ^ 0x5CA1_E123) * 0.30;
    let surface = refinement_height_detail(Vec2::new(local.x, local.z), seed);
    volumetric - surface
}
