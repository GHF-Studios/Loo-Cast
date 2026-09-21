//! Procedural backing fields for voxel worlds.
//!
//! A base field is reconstructible and therefore does not need to be persisted
//! per voxel. Persistent/world edits are layered on top by [`VoxelWorld`].

use bevy::prelude::Vec3;

use crate::spatial::UsfPosition;

use super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};

mod noise;
mod terrain;
mod celestial;
mod volume;

pub use celestial::ProceduralCelestialBody;
pub use terrain::ProceduralTerrain;
pub use volume::ProceduralVolume;

use volume::PreparedProceduralVolume;

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
    /// Spherical celestial-body baseline sampled by the ordinary voxel pipeline.
    CelestialBody(ProceduralCelestialBody),
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

    pub const fn celestial_body(body: ProceduralCelestialBody) -> Self {
        Self::CelestialBody(body)
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
            Self::CelestialBody(body) => body.sample_at(world_origin, point),
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
            Self::CelestialBody(body) => body
                .prepare_local_sampler(world_origin, chunk_origin)
                .map(PreparedVoxelBase::CelestialBody)
                .unwrap_or(PreparedVoxelBase::Canonical {
                    base: self,
                    world_origin,
                    chunk_origin,
                }),
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
    CelestialBody(celestial::PreparedProceduralCelestialBody),
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
            Self::CelestialBody(body) => body.sample(local),
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

#[cfg(test)]
use crate::spatial::{SPATIAL_SCALE_MAX, SpatialScale};
#[cfg(test)]
use noise::{semantic_value_noise, volumetric_noise};

#[cfg(test)]
mod tests;
