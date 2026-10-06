//! Dense bounded working materialization of a voxel field.
//!
//! ## Module map
//!
//! - `editing`: Projection/application of canonical semantic edits into dense chunk-local
//!   storage.
//! - `raycast`: Chunk-local signed-distance ray traversal and surface-crossing refinement.
//! - `sampling`: Dense chunk-local sample lookup, interpolation and storage addressing.
//!
//! This module groups the children; follow each child for its concrete implementation.
//!

use std::sync::Arc;

use bevy::prelude::{IVec3, UVec3, Vec3};

use super::{
    SignedDistance, VoxelEdit, VoxelMaterialId, VoxelMaterializationChunkAddress, VoxelSample,
};

/// Logical cell extent of one base voxel materialization chunk axis.
///
/// This decimal `10³` extent is representation structure, not a USF Chunk.
pub const MATERIALIZATION_CHUNK_SIZE: u32 = 10;

/// Neighbor samples retained around the logical chunk for seamless extraction.
/// Padding is private representation storage and is not part of chunk identity.
pub(crate) const SAMPLE_PADDING: u32 = 1;
pub(crate) const SAMPLE_SIZE: u32 = MATERIALIZATION_CHUNK_SIZE + SAMPLE_PADDING * 2;
const SAMPLE_COUNT: usize = (SAMPLE_SIZE * SAMPLE_SIZE * SAMPLE_SIZE) as usize;

const RAY_STEP: f32 = 0.25;
const RAY_REFINEMENT_STEPS: usize = 6;

fn detect_surface_transition(distances: &[f32]) -> bool {
    let Some((&first, rest)) = distances.split_first() else {
        return false;
    };
    let first_solid = first < 0.0;
    rest.iter().any(|&distance| (distance < 0.0) != first_solid)
}

/// Result of applying one semantic edit to a chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoxelMaterializationEditResult {
    pub changed_samples: usize,
    pub revision: u64,
}

impl VoxelMaterializationEditResult {
    pub const fn changed(self) -> bool {
        self.changed_samples != 0
    }
}

/// Ray hit expressed in the dense chunk's bounded local chart.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelRayHit {
    pub position: Vec3,
    pub distance: f32,
}

/// Dense sampled volume used as a disposable active working representation.
///
/// Every coordinate stored here is chunk-local. Samples cover `[-1, 10]` on
/// each axis because Surface Nets keeps one copied neighbor sample around the
/// logical half-open `[0, 10)³` ownership extent. Canonical semantic location
/// lives in the materialization store's canonical address key, never in this data.
#[derive(Debug, Clone)]
pub struct DenseVoxelMaterialization {
    // Worker snapshots share dense arrays. Edits use copy-on-write, making
    // queue/handoff clones O(1) while mutation remains locally owned.
    distances: Arc<[f32]>,
    materials: Arc<[VoxelMaterialId]>,
    surface_transition: bool,
    revision: u64,
    meshed_revision: Option<u64>,
}

impl DenseVoxelMaterialization {
    pub fn generate(mut generator: impl FnMut(Vec3) -> VoxelSample) -> Self {
        let mut distances = Vec::with_capacity(SAMPLE_COUNT);
        let mut materials = Vec::with_capacity(SAMPLE_COUNT);
        let padding = IVec3::splat(SAMPLE_PADDING as i32);
        let mut first_solid = None;
        let mut surface_transition = false;

        for z in 0..SAMPLE_SIZE {
            for y in 0..SAMPLE_SIZE {
                for x in 0..SAMPLE_SIZE {
                    let storage = IVec3::new(x as i32, y as i32, z as i32);
                    let local = storage - padding;
                    let sample = generator(local.as_vec3());
                    let solid = sample.distance.0 < 0.0;
                    match first_solid {
                        Some(first) => surface_transition |= first != solid,
                        None => first_solid = Some(solid),
                    }
                    distances.push(sample.distance.0);
                    materials.push(sample.material);
                }
            }
        }

        Self {
            distances: Arc::from(distances),
            materials: Arc::from(materials),
            surface_transition,
            revision: 0,
            meshed_revision: None,
        }
    }

    pub fn filled(sample: VoxelSample) -> Self {
        Self {
            distances: Arc::from(vec![sample.distance.0; SAMPLE_COUNT]),
            materials: Arc::from(vec![sample.material; SAMPLE_COUNT]),
            surface_transition: false,
            revision: 0,
            meshed_revision: None,
        }
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub const fn meshed_revision(&self) -> Option<u64> {
        self.meshed_revision
    }

    pub fn needs_remesh(&self) -> bool {
        self.meshed_revision != Some(self.revision)
    }

    pub fn mark_meshed(&mut self) {
        self.meshed_revision = Some(self.revision);
    }

    pub fn distances(&self) -> &[f32] {
        &self.distances
    }

    pub fn materials(&self) -> &[VoxelMaterialId] {
        &self.materials
    }

    #[inline]
    pub fn has_surface_transition(&self) -> bool {
        self.surface_transition
    }
}

mod editing;
mod raycast;
mod sampling;
