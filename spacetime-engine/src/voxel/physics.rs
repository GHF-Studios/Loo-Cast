//! Disposable collision representation derived from authoritative voxel chunks.
//!
//! Physics intentionally does not read the render [`Mesh`] asset back. Both
//! render and collision representations are built from the same extracted
//! surface, so either cache can diverge later (for example lower-detail physics)
//! without changing voxel-world semantics.

use avian3d::{collision::collider::TrimeshFlags, prelude::Collider};
use bevy::prelude::*;

use super::{CHUNK_SIZE, VoxelChunk, mesh::VoxelSurface};

/// Small thickness around the otherwise hollow terrain trimesh. This reduces
/// tunnelling and visible/contact jitter for character and rigid-body motion.
pub const VOXEL_COLLISION_MARGIN: f32 = 0.02;

pub(crate) fn build_chunk_collider(
    chunk: &VoxelChunk,
    surface: &VoxelSurface,
) -> Option<Collider> {
    let triangles = owned_triangles(surface);
    if triangles.is_empty() {
        return None;
    }

    let vertices = surface
        .positions
        .iter()
        .copied()
        .map(Vec3::from_array)
        .collect::<Vec<_>>();

    match Collider::try_trimesh_with_config(
        vertices,
        triangles,
        TrimeshFlags::FIX_INTERNAL_EDGES,
    ) {
        Ok(collider) => Some(collider),
        Err(error) => {
            warn!(
                ?error,
                origin = ?chunk.origin(),
                "failed to build voxel chunk collider"
            );
            None
        }
    }
}

/// Surface Nets needs one sample of neighbor padding, so extracted surfaces from
/// adjacent chunks overlap slightly. Rendering tolerates that, physics should
/// not: duplicated triangles make contacts at chunk seams considerably noisier.
///
/// Assign each triangle to exactly one brick by its brick-local centroid. A
/// triangle may cross the brick boundary, but only one collider owns it.
fn owned_triangles(surface: &VoxelSurface) -> Vec<[u32; 3]> {
    surface
        .indices
        .chunks_exact(3)
        .filter_map(|triangle| {
            let indices = [triangle[0], triangle[1], triangle[2]];
            let a = Vec3::from_array(surface.positions[indices[0] as usize]);
            let b = Vec3::from_array(surface.positions[indices[1] as usize]);
            let c = Vec3::from_array(surface.positions[indices[2] as usize]);
            let centroid = (a + b + c) / 3.0;

            owns_point(centroid).then_some(indices)
        })
        .collect()
}

fn owns_point(point: Vec3) -> bool {
    let maximum = Vec3::splat(CHUNK_SIZE as f32);
    point.cmpge(Vec3::ZERO).all() && point.cmplt(maximum).all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_ownership_is_half_open_and_unambiguous_at_seams() {
        let size = CHUNK_SIZE as f32;
        assert!(owns_point(Vec3::new(size - 0.001, 1.0, 1.0)));
        assert!(!owns_point(Vec3::new(size, 1.0, 1.0)));
        assert!(owns_point(Vec3::ZERO));
    }
}
