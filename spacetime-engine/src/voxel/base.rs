//! Procedural backing fields for voxel worlds.
//!
//! A base field is reconstructible and therefore does not need to be persisted
//! per voxel. Persistent/world edits are layered on top by [`VoxelWorld`].

use bevy::prelude::{Vec2, Vec3};

use crate::spatial::{SPATIAL_SCALE_MAX, SpatialScale, UsfPosition};

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
            Self::Terrain(terrain) => terrain.sample_at(world_origin, point),
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
