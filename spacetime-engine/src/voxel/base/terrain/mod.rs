//! Reference/legacy procedural heightfield backing a voxel base.

use bevy::prelude::Vec2;

use crate::spatial::UsfPosition;

use super::{
    EMPTY_DISTANCE, TERRAIN_DIRECT_LOCAL_LIMIT, TERRAIN_VERTICAL_QUERY_LIMIT,
    noise::{canonical_cell_size, semantic_value_noise, value_noise},
};
use super::super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};

/// Small dependency-free terrain field for the first procedural-world map.
///
/// Near the world origin this deliberately preserves the existing prototype
/// noise exactly. At very large travel distances it switches to a canonical
/// semantic lattice instead of feeding an ever-growing world coordinate into
/// `f32`; that fallback is temporary world-generation scaffolding, not a final
/// USF Phenomenon algorithm.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProceduralTerrain {
    pub seed: u32,
    pub base_height: f32,
    pub amplitude: f32,
    pub frequency: f32,
}

impl ProceduralTerrain {
    pub const fn new(seed: u32) -> Self {
        Self {
            seed,
            base_height: -4.0,
            amplitude: 3.0,
            frequency: 0.035,
        }
    }

    /// Temporary realization profile derived from Scale-0 semantic world state.
    ///
    /// This keeps the current voxel generator replaceable: geology/worldgen owns
    /// the semantic parameters while `ProceduralTerrain` only turns them into a
    /// reconstructible field until the richer volumetric realizer replaces it.
    pub const fn configured(seed: u32, base_height: f32, amplitude: f32, frequency: f32) -> Self {
        Self {
            seed,
            base_height,
            amplitude,
            frequency,
        }
    }

    /// Existing bounded/local prototype terrain function.
    pub fn height(self, x: f32, z: f32) -> f32 {
        let p = Vec2::new(x, z) * self.frequency.max(f32::EPSILON);
        let broad = value_noise(p, self.seed);
        let medium = value_noise(p * 2.13 + Vec2::new(17.0, -9.0), self.seed ^ 0xA511_E9B3);
        let fine = value_noise(p * 4.71 + Vec2::new(-31.0, 23.0), self.seed ^ 0x63D8_35A7);

        self.base_height + self.amplitude.max(0.0) * (broad + medium * 0.35 + fine * 0.12)
    }

    /// Terrain height in bounded units relative to `world_origin`.
    pub fn height_at(self, world_origin: VoxelQueryPosition, point: VoxelQueryPosition) -> f32 {
        // Keep the existing terrain bit-for-bit in the ordinary gameplay region.
        // The bound is policy for this temporary generator, not semantic space.
        if let Ok(local) = point.relative_to(world_origin, TERRAIN_DIRECT_LOCAL_LIMIT) {
            return self.height(local.x, local.z);
        }

        let frequency = self.frequency.max(f32::EPSILON);
        let broad = semantic_value_noise(point, canonical_cell_size(1.0 / frequency), self.seed);
        let medium = semantic_value_noise(
            point,
            canonical_cell_size(1.0 / (frequency * 2.13)),
            self.seed ^ 0xA511_E9B3,
        );
        let fine = semantic_value_noise(
            point,
            canonical_cell_size(1.0 / (frequency * 4.71)),
            self.seed ^ 0x63D8_35A7,
        );

        self.base_height + self.amplitude.max(0.0) * (broad + medium * 0.35 + fine * 0.12)
    }

    pub fn sample(self, point: VoxelQueryPosition) -> VoxelSample {
        self.sample_at(VoxelQueryPosition::new(UsfPosition::default()), point)
    }

    pub(crate) fn sample_at(
        self,
        world_origin: VoxelQueryPosition,
        point: VoxelQueryPosition,
    ) -> VoxelSample {
        // This is a height-field scalar rather than an exact Euclidean SDF.
        // Surface Nets only needs a smooth sign-changing scalar field here.
        let height = self.height_at(world_origin, point);
        let Some(y) = point
            .usf()
            .relative_native_axis_bounded(&world_origin.usf(), 1, TERRAIN_VERTICAL_QUERY_LIMIT)
            .ok()
        else {
            return if point
                .usf()
                .relative_native_axis_is_negative(&world_origin.usf(), 1)
                .unwrap_or(false)
            {
                VoxelSample::new(-EMPTY_DISTANCE, VoxelMaterialId::ROCK)
            } else {
                VoxelSample::empty(EMPTY_DISTANCE)
            };
        };
        let distance = y - height;
        VoxelSample::new(
            distance,
            if distance < 0.0 {
                VoxelMaterialId::ROCK
            } else {
                VoxelMaterialId::VOID
            },
        )
    }
}
