//! Editable volumetric world primitives.
//!
//! Procedural base + sparse modifications are authoritative. Dense chunks are
//! only materialized working caches; rendering and physics are disposable
//! representations rebuilt from those chunks as the active window streams.

mod base;
mod chunk;
mod edit;
mod field;
mod mesh;
mod modification;
mod physics;
mod reconcile;
mod streaming;
mod world;

pub use base::{ProceduralTerrain, VoxelBase};
pub use chunk::{
    CHUNK_SIZE, SAMPLE_COUNT, SAMPLE_PADDING, SAMPLE_SIZE, VoxelChunk, VoxelChunkEditResult,
    VoxelRayHit,
};
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit};
pub use field::{SignedDistance, VoxelMaterialId, VoxelSample};
pub use mesh::VoxelRenderMesh;
pub use modification::VoxelModificationLayer;
pub use streaming::VoxelStreaming;
pub use world::{VoxelChunkCoord, VoxelChunkOf, VoxelWorld};

use bevy::prelude::*;

use crate::physics::character::CharacterMovementSet;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, streaming::stream_voxel_chunks)
            .add_systems(
                FixedUpdate,
                reconcile::reconcile_voxel_characters.after(CharacterMovementSet::Simulate),
            )
            .add_systems(PostUpdate, mesh::rebuild_dirty_chunks);
    }
}
