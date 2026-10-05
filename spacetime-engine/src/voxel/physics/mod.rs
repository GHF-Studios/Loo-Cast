//! Disposable collision representation derived from authoritative voxel chunks.
//!
//! Physics intentionally does not read the render [`Mesh`] asset back. Both
//! render and collision representations are built from the same extracted
//! surface, so either cache can diverge later (for example lower-detail physics)
//! without changing voxel-world semantics.

use avian3d::{collision::collider::TrimeshFlags, prelude::Collider};
use bevy::prelude::*;

use super::mesh::VoxelSurface;

/// Physical thickness around the otherwise hollow terrain trimesh.
///
/// This is semantic contact policy in SI metres, not Scale-Slice-native units.
/// Each collision realization converts it at its backend boundary.
pub(super) const VOXEL_COLLISION_MARGIN_METRES: f32 = 0.02;

pub(super) fn build_trimesh_collider(
    vertices: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    context: &'static str,
) -> Option<Collider> {
    if triangles.is_empty() {
        return None;
    }

    match Collider::try_trimesh_with_config(vertices, triangles, TrimeshFlags::FIX_INTERNAL_EDGES) {
        Ok(collider) => Some(collider),
        Err(error) => {
            warn!(?error, context, "failed to build voxel trimesh collider");
            None
        }
    }
}

/// Rigid topology emitted by Surface Nets for this materialization.
///
/// The extractor already assigns shared padded-chunk faces by omitting
/// positive-boundary faces. Collision must consume exactly that topology rather
/// than applying a second geometric ownership rule.
pub(super) fn triangles(surface: &VoxelSurface) -> Vec<[u32; 3]> {
    surface.rigid_triangles()
}
