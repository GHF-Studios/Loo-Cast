//! Reconstructible three-dimensional procedural voxel field.

use bevy::prelude::{Vec2, Vec3};

use crate::spatial::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale,
    UsfPosition,
};

use super::{
    EMPTY_DISTANCE, TERRAIN_DIRECT_LOCAL_LIMIT, TERRAIN_VERTICAL_QUERY_LIMIT,
    noise::{mix, scale_layer_seed, value_noise, value_noise_3d, volumetric_noise},
    terrain::ProceduralTerrain,
};
use super::super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};

mod refinement;

use refinement::{
    ScaleRefinementHierarchy, refinement_surface_height, sample_refinement_hierarchy,
};

/// Reconstructible three-dimensional local matter field.
///
/// A planetary/reference surface is one contextual input, not the authority.
/// 3D structural noise and cave fields participate directly in the signed scalar
/// field, so caves, arches and overhangs are first-class local geometry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProceduralVolume {
    surface: ProceduralTerrain,
    cave_strength: f32,
    structure_strength: f32,
    parent_macro_seed: Option<u32>,
    parent_macro_origin: f32,
    hierarchy: Option<ScaleRefinementHierarchy>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedProceduralVolume {
    volume: ProceduralVolume,
    chunk_origin_from_world: Vec3,
}

impl PreparedProceduralVolume {
    #[inline]
    pub(crate) fn sample(self, chunk_local: Vec3) -> VoxelSample {
        self.volume
            .sample_local(self.chunk_origin_from_world + chunk_local)
    }
}

impl ProceduralVolume {
    pub fn configured(
        seed: u32,
        base_height: f32,
        amplitude: f32,
        frequency: f32,
        cave_strength: f32,
        structure_strength: f32,
    ) -> Self {
        Self {
            surface: ProceduralTerrain::configured(seed, base_height, amplitude, frequency),
            cave_strength: cave_strength.clamp(0.0, 1.0),
            structure_strength: structure_strength.clamp(0.0, 1.0),
            parent_macro_seed: None,
            parent_macro_origin: 0.0,
            hierarchy: None,
        }
    }

    pub fn scale_layer(universe_seed: u64, context_seed: u64, scale: SpatialScale) -> Self {
        Self::scale_refinement(universe_seed, scale, &[(scale, context_seed)])
    }

    /// Testing realizer for the USF refinement spine.
    ///
    /// Every finer scale reproduces the entire already-resolved coarser field in
    /// its own native units, then adds only detail native to newly-entered scales.
    /// One +35 unit therefore becomes ten +34 units with the same broad geometry.
    pub fn scale_refinement(
        universe_seed: u64,
        scale: SpatialScale,
        lineage: &[(SpatialScale, u64)],
    ) -> Self {
        let hierarchy_seed = (universe_seed as u32) ^ ((universe_seed >> 32) as u32);
        let mut seeds = [0_u32; SPATIAL_SCALE_COUNT];

        for raw in SPATIAL_SCALE_MIN..=SPATIAL_SCALE_MAX {
            let level = SpatialScale::new(raw).expect("validated spatial scale");
            seeds[level.index_from_top()] = scale_layer_seed(hierarchy_seed, level);
        }

        for &(level, context_seed) in lineage {
            let folded = (context_seed as u32) ^ ((context_seed >> 32) as u32);
            seeds[level.index_from_top()] = mix(seeds[level.index_from_top()], folded);
        }

        let detail_seed = seeds[scale.index_from_top()];
        let unit = detail_seed as f32 / u32::MAX as f32;

        Self {
            surface: ProceduralTerrain::configured(detail_seed, -4.0, 3.0, 0.035),
            cave_strength: 0.10 + unit * 0.24,
            structure_strength: 0.35 + unit * 0.50,
            parent_macro_seed: None,
            parent_macro_origin: 0.0,
            hierarchy: Some(ScaleRefinementHierarchy {
                current_scale: scale,
                seeds,
            }),
        }
    }

    pub const fn reference_surface(self) -> ProceduralTerrain {
        self.surface
    }

    pub(crate) fn prepare_local_sampler(
        self,
        world_origin: VoxelQueryPosition,
        chunk_origin: VoxelQueryPosition,
    ) -> Option<PreparedProceduralVolume> {
        let local = chunk_origin
            .relative_to(world_origin, TERRAIN_DIRECT_LOCAL_LIMIT - 64.0)
            .ok()?;
        if local.y.abs() > TERRAIN_VERTICAL_QUERY_LIMIT - 64.0 {
            return None;
        }
        Some(PreparedProceduralVolume {
            volume: self,
            chunk_origin_from_world: local,
        })
    }

    #[inline]
    fn sample_local(self, local: Vec3) -> VoxelSample {
        if let Some(hierarchy) = self.hierarchy {
            return sample_refinement_hierarchy(local, hierarchy);
        }

        let surface_height = self.surface_height_local(local.x, local.z);
        let base_frequency = self.surface.frequency.max(f32::EPSILON);
        let relief = self.surface.amplitude.max(1.0);

        let structure = value_noise_3d(
            local * (base_frequency * 0.72),
            self.surface.seed ^ 0x31D0_6A5B,
        );
        let structure_offset = structure * relief * (0.18 + self.structure_strength * 0.82);
        let exterior_distance = local.y - surface_height + structure_offset;

        let cave_frequency = base_frequency * (1.65 + self.cave_strength * 1.35);
        let cave_noise = value_noise_3d(local * cave_frequency, self.surface.seed ^ 0xCA7E_5EED);
        let cave_half_width = 0.045 + self.cave_strength * 0.18;
        let depth = (surface_height - local.y).max(0.0);
        let underground_gate = (depth / 3.0).clamp(0.0, 1.0);
        let cave_distance =
            (cave_half_width - cave_noise.abs()) * (4.0 + relief * 0.35) * underground_gate;

        let distance = exterior_distance.max(cave_distance);
        VoxelSample::new(
            distance,
            if distance < 0.0 {
                VoxelMaterialId::ROCK
            } else {
                VoxelMaterialId::VOID
            },
        )
    }

    fn surface_height_local(self, x: f32, z: f32) -> f32 {
        if let Some(hierarchy) = self.hierarchy {
            return refinement_surface_height(Vec2::new(x, z), hierarchy);
        }

        let mut height = self.surface.height(x, z);
        if let Some(parent_seed) = self.parent_macro_seed {
            let parent_point = Vec2::new(x, z) * (self.surface.frequency.max(f32::EPSILON) / 10.0);
            let inherited = value_noise(parent_point, parent_seed) - self.parent_macro_origin;
            height += self.surface.amplitude * 4.0 * inherited;
        }
        height
    }

    /// Approximate exterior surface used only for spawn placement and coarse
    /// surface presentation. It is not the volumetric matter authority.
    pub fn reference_surface_height_at(
        self,
        world_origin: VoxelQueryPosition,
        point: VoxelQueryPosition,
    ) -> f32 {
        if let Ok(local) = point.relative_to(world_origin, TERRAIN_DIRECT_LOCAL_LIMIT) {
            self.surface_height_local(local.x, local.z)
        } else {
            self.surface.height_at(world_origin, point)
        }
    }

    pub fn sample(self, point: VoxelQueryPosition) -> VoxelSample {
        self.sample_at(VoxelQueryPosition::new(UsfPosition::default()), point)
    }

    pub(crate) fn sample_at(
        self,
        world_origin: VoxelQueryPosition,
        point: VoxelQueryPosition,
    ) -> VoxelSample {
        if self.hierarchy.is_some() {
            if let Ok(local) = point.relative_to(world_origin, TERRAIN_DIRECT_LOCAL_LIMIT) {
                return self.sample_local(local);
            }
        }

        let surface_height = self.reference_surface_height_at(world_origin, point);
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

        let base_frequency = self.surface.frequency.max(f32::EPSILON);
        let structure = volumetric_noise(
            world_origin,
            point,
            base_frequency * 0.72,
            self.surface.seed ^ 0x31D0_6A5B,
        );
        let relief = self.surface.amplitude.max(1.0);
        let structure_offset = structure * relief * (0.18 + self.structure_strength * 0.82);

        // Negative is matter. Since `structure` varies with x/y/z, the local
        // boundary cannot be represented as y = h(x,z).
        let exterior_distance = y - surface_height + structure_offset;

        // Carve connected 3D cave bands inside the body. `max` performs signed
        // field subtraction here: positive cave field wins over negative solid.
        let cave_frequency = base_frequency * (1.65 + self.cave_strength * 1.35);
        let cave_noise = volumetric_noise(
            world_origin,
            point,
            cave_frequency,
            self.surface.seed ^ 0xCA7E_5EED,
        );
        let cave_half_width = 0.045 + self.cave_strength * 0.18;
        let depth = (surface_height - y).max(0.0);
        let underground_gate = (depth / 3.0).clamp(0.0, 1.0);
        let cave_distance =
            (cave_half_width - cave_noise.abs()) * (4.0 + relief * 0.35) * underground_gate;

        let distance = exterior_distance.max(cave_distance);
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
