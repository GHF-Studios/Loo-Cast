//! Procedural backing fields for voxel worlds.
//!
//! A base field is reconstructible and therefore does not need to be persisted
//! per voxel. Persistent/world edits are layered on top by [`VoxelWorld`].

use bevy::prelude::{Vec2, Vec3};

use crate::spatial::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale, UsfPosition,
};

use super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};

const EMPTY_DISTANCE: f32 = 1024.0;
const TERRAIN_VERTICAL_QUERY_LIMIT: f32 = 4096.0;
const TERRAIN_DIRECT_LOCAL_LIMIT: f32 = 1_000_000.0;

/// Reconstructible source field underneath sparse voxel modifications.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoxelBase {
    Empty,
    Sphere {
        center: VoxelQueryPosition,
        radius: f32,
        material: VoxelMaterialId,
    },
    /// Active local-world base: genuine 3D density/material field.
    Volume(ProceduralVolume),
    /// Legacy/reference heightfield retained for tests and compatibility.
    Terrain(ProceduralTerrain),
}

impl Default for VoxelBase {
    fn default() -> Self {
        Self::Empty
    }
}

impl VoxelBase {
    pub fn sphere(center: VoxelQueryPosition, radius: f32, material: VoxelMaterialId) -> Self {
        Self::Sphere {
            center,
            radius: radius.max(0.0),
            material,
        }
    }

    pub const fn terrain(seed: u32) -> Self {
        Self::Terrain(ProceduralTerrain::new(seed))
    }

    pub fn sample(self, point: VoxelQueryPosition) -> VoxelSample {
        self.sample_in_world(VoxelQueryPosition::new(UsfPosition::default()), point)
    }

    pub(crate) fn sample_in_world(
        self,
        world_origin: VoxelQueryPosition,
        point: VoxelQueryPosition,
    ) -> VoxelSample {
        match self {
            Self::Empty => VoxelSample::empty(EMPTY_DISTANCE),
            Self::Sphere {
                center,
                radius,
                material,
            } => {
                let Ok(delta) = point.relative_to(center, radius + EMPTY_DISTANCE) else {
                    return VoxelSample::empty(EMPTY_DISTANCE);
                };
                let distance = delta.length() - radius;
                VoxelSample::new(
                    distance,
                    if distance < 0.0 {
                        material
                    } else {
                        VoxelMaterialId::VOID
                    },
                )
            }
            Self::Volume(volume) => volume.sample_at(world_origin, point),
            Self::Terrain(terrain) => terrain.sample_at(world_origin, point),
        }
    }

    pub(crate) fn prepare_chunk_sampler(
        self,
        world_origin: VoxelQueryPosition,
        chunk_origin: VoxelQueryPosition,
    ) -> PreparedVoxelBase {
        match self {
            Self::Volume(volume) => volume
                .prepare_local_sampler(world_origin, chunk_origin)
                .map(PreparedVoxelBase::LocalVolume)
                .unwrap_or(PreparedVoxelBase::Canonical {
                    base: self,
                    world_origin,
                    chunk_origin,
                }),
            _ => PreparedVoxelBase::Canonical {
                base: self,
                world_origin,
                chunk_origin,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum PreparedVoxelBase {
    LocalVolume(PreparedProceduralVolume),
    Canonical {
        base: VoxelBase,
        world_origin: VoxelQueryPosition,
        chunk_origin: VoxelQueryPosition,
    },
}

impl PreparedVoxelBase {
    #[inline]
    pub(crate) fn sample(self, local: Vec3) -> VoxelSample {
        match self {
            Self::LocalVolume(volume) => volume.sample(local),
            Self::Canonical {
                base,
                world_origin,
                chunk_origin,
            } => {
                let Ok(point) = chunk_origin.translated(local) else {
                    return VoxelSample::empty(f32::INFINITY);
                };
                base.sample_in_world(world_origin, point)
            }
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScaleRefinementHierarchy {
    current_scale: SpatialScale,
    seeds: [u32; SPATIAL_SCALE_COUNT],
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

fn sample_refinement_hierarchy(local: Vec3, hierarchy: ScaleRefinementHierarchy) -> VoxelSample {
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

fn refinement_surface_height(local: Vec2, hierarchy: ScaleRefinementHierarchy) -> f32 {
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

fn volumetric_noise(
    world_origin: VoxelQueryPosition,
    point: VoxelQueryPosition,
    frequency: f32,
    seed: u32,
) -> f32 {
    let frequency = frequency.max(f32::EPSILON);
    if let Ok(local) = point.relative_to(world_origin, TERRAIN_DIRECT_LOCAL_LIMIT) {
        value_noise_3d(local * frequency, seed)
    } else {
        semantic_value_noise_3d(point, canonical_cell_size(1.0 / frequency), seed)
    }
}

fn value_noise_3d(point: Vec3, seed: u32) -> f32 {
    let cell = point.floor().as_ivec3();
    let fraction = point - cell.as_vec3();
    let smooth = fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);
    let corner =
        |dx: i32, dy: i32, dz: i32| hash_noise_3d(cell.x + dx, cell.y + dy, cell.z + dz, seed);

    let c000 = corner(0, 0, 0);
    let c100 = corner(1, 0, 0);
    let c010 = corner(0, 1, 0);
    let c110 = corner(1, 1, 0);
    let c001 = corner(0, 0, 1);
    let c101 = corner(1, 0, 1);
    let c011 = corner(0, 1, 1);
    let c111 = corner(1, 1, 1);
    let x00 = c000 + (c100 - c000) * smooth.x;
    let x10 = c010 + (c110 - c010) * smooth.x;
    let x01 = c001 + (c101 - c001) * smooth.x;
    let x11 = c011 + (c111 - c011) * smooth.x;
    let y0 = x00 + (x10 - x00) * smooth.y;
    let y1 = x01 + (x11 - x01) * smooth.y;
    y0 + (y1 - y0) * smooth.z
}

fn hash_noise_3d(x: i32, y: i32, z: i32, seed: u32) -> f32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9E37_79B9)
        ^ (y as u32).wrapping_mul(0x85EB_CA6B)
        ^ (z as u32).wrapping_mul(0xC2B2_AE35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^= value >> 16;
    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

fn semantic_value_noise_3d(point: VoxelQueryPosition, cell_size: i64, seed: u32) -> f32 {
    let size = cell_size as f32;
    let offset = point.usf().offset();
    let remainder = Vec3::new(
        offset.x.rem_euclid(size),
        offset.y.rem_euclid(size),
        offset.z.rem_euclid(size),
    );
    let fraction = remainder / size;
    let smooth = fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);
    let lower = point
        .translated(-remainder)
        .expect("bounded semantic 3D noise-lattice translation");

    let corner = |dx: i64, dy: i64, dz: i64| {
        let p = VoxelQueryPosition::new(
            lower
                .usf()
                .translated_whole_native([dx * cell_size, dy * cell_size, dz * cell_size])
                .expect("bounded semantic 3D noise-lattice translation"),
        );
        semantic_corner_noise_3d(p, seed)
    };

    let c000 = corner(0, 0, 0);
    let c100 = corner(1, 0, 0);
    let c010 = corner(0, 1, 0);
    let c110 = corner(1, 1, 0);
    let c001 = corner(0, 0, 1);
    let c101 = corner(1, 0, 1);
    let c011 = corner(0, 1, 1);
    let c111 = corner(1, 1, 1);
    let x00 = c000 + (c100 - c000) * smooth.x;
    let x10 = c010 + (c110 - c010) * smooth.x;
    let x01 = c001 + (c101 - c001) * smooth.x;
    let x11 = c011 + (c111 - c011) * smooth.x;
    let y0 = x00 + (x10 - x00) * smooth.y;
    let y1 = x01 + (x11 - x01) * smooth.y;
    y0 + (y1 - y0) * smooth.z
}

fn semantic_corner_noise_3d(point: VoxelQueryPosition, seed: u32) -> f32 {
    let position = point.usf();
    let mut value = seed ^ 0x517C_C1B7;
    for raw_scale in (position.leaf_scale().exponent()..=SPATIAL_SCALE_MAX).rev() {
        let scale = SpatialScale::new(raw_scale).expect("range is validated");
        let digit = position.digit(scale);
        value = mix(value, digit.x as u32);
        value = mix(value, digit.y as u32);
        value = mix(value, digit.z as u32);
    }
    let offset = position.offset();
    value = mix(value, canonical_f32_bits(offset.x));
    value = mix(value, canonical_f32_bits(offset.y));
    value = mix(value, canonical_f32_bits(offset.z));
    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

fn value_noise(point: Vec2, seed: u32) -> f32 {
    let cell = point.floor().as_ivec2();
    let fraction = point - cell.as_vec2();
    let smooth = fraction * fraction * (Vec2::splat(3.0) - fraction * 2.0);

    let a = hash_noise(cell.x, cell.y, seed);
    let b = hash_noise(cell.x + 1, cell.y, seed);
    let c = hash_noise(cell.x, cell.y + 1, seed);
    let d = hash_noise(cell.x + 1, cell.y + 1, seed);

    let x0 = a + (b - a) * smooth.x;
    let x1 = c + (d - c) * smooth.x;
    x0 + (x1 - x0) * smooth.y
}

fn hash_noise(x: i32, y: i32, seed: u32) -> f32 {
    let mut value =
        seed ^ (x as u32).wrapping_mul(0x9E37_79B9) ^ (y as u32).wrapping_mul(0x85EB_CA6B);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^= value >> 16;

    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

/// Snaps a requested fallback noise wavelength to an integer divisor of the
/// scale-0 USF chunk size, keeping all modular arithmetic bounded.
fn canonical_cell_size(target: f32) -> i64 {
    const DIVISORS: [i64; 14] = [1, 2, 4, 5, 8, 10, 20, 25, 40, 50, 100, 125, 250, 500];
    let target = target.clamp(1.0, 500.0);
    DIVISORS
        .into_iter()
        .min_by(|a, b| ((*a as f32 - target).abs()).total_cmp(&(*b as f32 - target).abs()))
        .unwrap()
}

fn semantic_value_noise(point: VoxelQueryPosition, cell_size: i64, seed: u32) -> f32 {
    let size = cell_size as f32;
    let offset = point.usf().offset();
    let remainder = Vec2::new(offset.x.rem_euclid(size), offset.z.rem_euclid(size));
    let fraction = remainder / size;
    let smooth = fraction * fraction * (Vec2::splat(3.0) - fraction * 2.0);

    // Every chosen cell size divides 1000 scale-0 native units, so subtracting
    // the leaf-offset remainder lands on one stable canonical lattice through
    // carries in higher USF digits.
    let lower = point
        .translated(Vec3::new(-remainder.x, 0.0, -remainder.y))
        .expect("bounded semantic noise-lattice translation");
    let x = VoxelQueryPosition::new(
        lower
            .usf()
            .translated_whole_native([cell_size, 0, 0])
            .expect("bounded semantic noise-lattice translation"),
    );
    let z = VoxelQueryPosition::new(
        lower
            .usf()
            .translated_whole_native([0, 0, cell_size])
            .expect("bounded semantic noise-lattice translation"),
    );
    let xz = VoxelQueryPosition::new(
        lower
            .usf()
            .translated_whole_native([cell_size, 0, cell_size])
            .expect("bounded semantic noise-lattice translation"),
    );

    let a = semantic_corner_noise(lower, seed);
    let b = semantic_corner_noise(x, seed);
    let c = semantic_corner_noise(z, seed);
    let d = semantic_corner_noise(xz, seed);
    let x0 = a + (b - a) * smooth.x;
    let x1 = c + (d - c) * smooth.x;
    x0 + (x1 - x0) * smooth.y
}

fn semantic_corner_noise(point: VoxelQueryPosition, seed: u32) -> f32 {
    let position = point.usf();
    let mut value = seed ^ 0x9E37_79B9;

    for raw_scale in (position.leaf_scale().exponent()..=SPATIAL_SCALE_MAX).rev() {
        let scale = SpatialScale::new(raw_scale).expect("range is validated");
        let digit = position.digit(scale);
        value = mix(value, digit.x as u32);
        value = mix(value, digit.z as u32);
    }

    value = mix(value, canonical_f32_bits(position.offset().x));
    value = mix(value, canonical_f32_bits(position.offset().z));
    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

fn scale_layer_seed(seed: u32, scale: SpatialScale) -> u32 {
    mix(
        seed ^ 0xA17E_5CA1,
        (scale.exponent() as i32 - crate::spatial::SPATIAL_SCALE_MIN as i32) as u32,
    )
}

fn mix(mut state: u32, input: u32) -> u32 {
    state ^= input.wrapping_mul(0x85EB_CA6B);
    state ^= state >> 16;
    state = state.wrapping_mul(0x7FEB_352D);
    state ^= state >> 15;
    state
}

fn canonical_f32_bits(value: f32) -> u32 {
    if value == 0.0 { 0 } else { value.to_bits() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(local: Vec3) -> VoxelQueryPosition {
        VoxelQueryPosition::from_scale0_local(local).unwrap()
    }

    #[test]
    fn procedural_terrain_preserves_existing_near_origin_field() {
        let terrain = ProceduralTerrain::new(42);
        let origin = query(Vec3::ZERO);
        let point = query(Vec3::new(123.5, 0.0, -87.25));

        assert_eq!(
            terrain.height_at(origin, point),
            terrain.height(123.5, -87.25)
        );
        assert_eq!(
            terrain.sample_at(origin, point),
            terrain.sample_at(origin, point)
        );
        assert_ne!(
            terrain.height_at(origin, point),
            ProceduralTerrain::new(43).height_at(origin, point)
        );
    }

    #[test]
    fn procedural_volume_structure_is_genuinely_three_dimensional() {
        let origin = query(Vec3::ZERO);
        let a = query(Vec3::new(17.0, -8.0, 23.0));
        let b = query(Vec3::new(17.0, 11.0, 23.0));
        let av = volumetric_noise(origin, a, 0.041, 77);
        let bv = volumetric_noise(origin, b, 0.041, 77);
        assert_ne!(av, bv);
    }

    #[test]
    fn child_scale_refines_parent_instead_of_regenerating_it() {
        let root_seed = 0x1234_5678_9ABC_DEF0_u64;
        let child_seed = 0x0FED_CBA9_8765_4321_u64;
        let parent_scale = SpatialScale::MAX;
        let child_scale = SpatialScale::new(SPATIAL_SCALE_MAX - 1).unwrap();

        let parent = ProceduralVolume::scale_refinement(
            0x10_0CA57_5EED_2026,
            parent_scale,
            &[(parent_scale, root_seed)],
        );
        let child = ProceduralVolume::scale_refinement(
            0x10_0CA57_5EED_2026,
            child_scale,
            &[(child_scale, child_seed), (parent_scale, root_seed)],
        );

        let parent_point = Vec3::new(7.25, -1.5, -3.75);
        let child_point = parent_point * 10.0;
        let parent_distance = parent.sample_local(parent_point).distance.0;
        let child_distance_in_parent_units = child.sample_local(child_point).distance.0 / 10.0;

        assert!(
            (parent_distance - child_distance_in_parent_units).abs() < 0.08,
            "parent={parent_distance}, child-as-parent={child_distance_in_parent_units}"
        );
    }

    #[test]
    fn sphere_base_is_reconstructible_without_stored_voxels() {
        let origin = query(Vec3::ZERO);
        let base = VoxelBase::sphere(origin, 2.0, VoxelMaterialId::ROCK);
        assert!(base.sample_in_world(origin, origin).distance.is_solid());
        assert!(
            base.sample_in_world(origin, query(Vec3::splat(4.0)))
                .distance
                .is_empty()
        );
    }

    #[test]
    fn canonical_fallback_noise_is_continuous_across_a_usf_digit_carry() {
        let left = query(Vec3::new(499.75, 0.0, 0.0));
        let right = left.translated(Vec3::new(0.5, 0.0, 0.0)).unwrap();
        let left_value = semantic_value_noise(left, 25, 42);
        let right_value = semantic_value_noise(right, 25, 42);

        assert!((left_value - right_value).abs() < 0.25);
    }
}
