//! Procedural backing fields for voxel worlds.
//!
//! A base field is reconstructible and therefore does not need to be persisted
//! per voxel. Persistent/world edits are layered on top by [`VoxelWorld`].

use bevy::prelude::{Vec2, Vec3};

use super::{VoxelMaterialId, VoxelSample};

const EMPTY_DISTANCE: f32 = 1024.0;

/// Reconstructible source field underneath sparse voxel modifications.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoxelBase {
    Empty,
    Sphere {
        center: Vec3,
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
    pub const fn sphere(center: Vec3, radius: f32, material: VoxelMaterialId) -> Self {
        Self::Sphere {
            center,
            radius,
            material,
        }
    }

    pub const fn terrain(seed: u32) -> Self {
        Self::Terrain(ProceduralTerrain::new(seed))
    }

    pub fn sample(self, point: Vec3) -> VoxelSample {
        match self {
            Self::Empty => VoxelSample::empty(EMPTY_DISTANCE),
            Self::Sphere {
                center,
                radius,
                material,
            } => {
                let distance = point.distance(center) - radius.max(0.0);
                VoxelSample::new(
                    distance,
                    if distance < 0.0 {
                        material
                    } else {
                        VoxelMaterialId::VOID
                    },
                )
            }
            Self::Terrain(terrain) => terrain.sample(point),
        }
    }
}

/// Small dependency-free terrain field for the first procedural-world map.
///
/// This is intentionally not our final world generator. It exists to prove the
/// important storage property: the base can be regenerated from a tiny seed
/// while edits remain independent sparse state.
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

    pub fn height(self, x: f32, z: f32) -> f32 {
        let p = Vec2::new(x, z) * self.frequency.max(f32::EPSILON);
        let broad = value_noise(p, self.seed);
        let medium = value_noise(p * 2.13 + Vec2::new(17.0, -9.0), self.seed ^ 0xA511_E9B3);
        let fine = value_noise(p * 4.71 + Vec2::new(-31.0, 23.0), self.seed ^ 0x63D8_35A7);

        self.base_height + self.amplitude.max(0.0) * (broad + medium * 0.35 + fine * 0.12)
    }

    pub fn sample(self, point: Vec3) -> VoxelSample {
        // This is a height-field scalar rather than an exact Euclidean SDF.
        // Surface Nets only needs a smooth sign-changing scalar field here.
        let distance = point.y - self.height(point.x, point.z);
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
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9E37_79B9)
        ^ (y as u32).wrapping_mul(0x85EB_CA6B);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^= value >> 16;

    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural_terrain_is_deterministic_from_its_seed() {
        let terrain = ProceduralTerrain::new(42);
        let point = Vec3::new(123.5, 0.0, -87.25);

        assert_eq!(terrain.sample(point), terrain.sample(point));
        assert_ne!(
            terrain.height(point.x, point.z),
            ProceduralTerrain::new(43).height(point.x, point.z)
        );
    }

    #[test]
    fn sphere_base_is_reconstructible_without_stored_voxels() {
        let base = VoxelBase::sphere(Vec3::ZERO, 2.0, VoxelMaterialId::ROCK);
        assert!(base.sample(Vec3::ZERO).distance.is_solid());
        assert!(base.sample(Vec3::splat(4.0)).distance.is_empty());
    }
}
