//! Disposable collision representation derived from authoritative voxel chunks.
//!
//! Physics intentionally does not read the render [`Mesh`] asset back. Both
//! render and collision representations are built from the same extracted
//! surface, so either cache can diverge later (for example lower-detail physics)
//! without changing voxel-world semantics.

use avian3d::{collision::collider::TrimeshFlags, prelude::Collider};
use bevy::prelude::*;

use super::mesh::VoxelSurface;

/// Small thickness around the otherwise hollow terrain trimesh. This reduces
/// tunnelling and visible/contact jitter for character and rigid-body motion.
pub(super) const VOXEL_COLLISION_MARGIN: f32 = 0.02;

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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::voxel::{
        MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelMaterialId, VoxelSample,
        mesh::extract_chunk_surface,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct QuantizedPoint(i32, i32, i32);

    type TriangleKey = [QuantizedPoint; 3];

    #[test]
    fn collisionless_nebula_has_no_collision_triangles() {
        let center = Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5);
        let chunk = VoxelChunk::generate(|local| {
            let distance = local.distance(center) - 3.0;
            VoxelSample::new(
                distance,
                if distance < 0.0 {
                    VoxelMaterialId::NEBULA
                } else {
                    VoxelMaterialId::VOID
                },
            )
        });
        let surface = extract_chunk_surface(&chunk);
        assert!(!surface.positions.is_empty());
        assert!(triangles(&surface).is_empty());
    }

    #[test]
    fn neighboring_surface_nets_chunks_use_native_seam_partition() {
        let size = MATERIALIZATION_CHUNK_SIZE as f32;
        let center = Vec3::new(size, size * 0.5, size * 0.5);
        let radius = size * 0.4;
        let sample = |world: Vec3| {
            let distance = world.distance(center) - radius;
            VoxelSample::new(
                distance,
                if distance < 0.0 {
                    VoxelMaterialId::ROCK
                } else {
                    VoxelMaterialId::VOID
                },
            )
        };

        let left = VoxelChunk::generate(|local| sample(local));
        let right_origin = Vec3::new(size, 0.0, 0.0);
        let right = VoxelChunk::generate(|local| sample(right_origin + local));
        let left_surface = extract_chunk_surface(&left);
        let right_surface = extract_chunk_surface(&right);

        assert!(build_test_surface_collider(&left_surface).is_some());
        assert!(build_test_surface_collider(&right_surface).is_some());

        // The positive neighbor legitimately owns seam topology whose centroid
        // lives in its negative padding domain. The old [0,10) centroid clip
        // deleted these triangles.
        let right_negative_padding = triangles(&right_surface)
            .into_iter()
            .filter(|indices| triangle_centroid(&right_surface, *indices).x < 0.0)
            .collect::<Vec<_>>();
        assert!(
            !right_negative_padding.is_empty(),
            "test surface must exercise native negative-padding seam ownership",
        );

        // Surface Nets' built-in sparse-chunk partition should not duplicate
        // exact shared-face triangles across the two neighboring chunks.
        let mut seen = BTreeSet::<TriangleKey>::new();
        let mut seam_count = 0usize;
        for (surface, world_offset) in [
            (&left_surface, Vec3::ZERO),
            (&right_surface, right_origin),
        ] {
            for indices in triangles(surface) {
                let centroid = triangle_centroid(surface, indices) + world_offset;
                if (centroid.x - size).abs() > 1.25
                    || centroid.y <= 1.0
                    || centroid.y >= size - 1.0
                    || centroid.z <= 1.0
                    || centroid.z >= size - 1.0
                {
                    continue;
                }
                seam_count += 1;
                let key = triangle_key(surface, indices, world_offset);
                assert!(
                    seen.insert(key),
                    "duplicate native Surface Nets seam triangle: {key:?}",
                );
            }
        }
        assert!(seam_count > 0, "test sphere must cross the materialization seam");
    }

    fn build_test_surface_collider(surface: &VoxelSurface) -> Option<Collider> {
        let vertices = surface
            .positions
            .iter()
            .copied()
            .map(Vec3::from_array)
            .collect();
        build_trimesh_collider(
            vertices,
            triangles(surface),
            "voxel physics native seam test",
        )
    }

    fn triangle_centroid(surface: &VoxelSurface, indices: [u32; 3]) -> Vec3 {
        let a = Vec3::from_array(surface.positions[indices[0] as usize]);
        let b = Vec3::from_array(surface.positions[indices[1] as usize]);
        let c = Vec3::from_array(surface.positions[indices[2] as usize]);
        (a + b + c) / 3.0
    }

    fn triangle_key(
        surface: &VoxelSurface,
        indices: [u32; 3],
        world_offset: Vec3,
    ) -> TriangleKey {
        let mut points = indices.map(|index| {
            let point =
                Vec3::from_array(surface.positions[index as usize]) + world_offset;
            QuantizedPoint(
                (point.x * 10_000.0).round() as i32,
                (point.y * 10_000.0).round() as i32,
                (point.z * 10_000.0).round() as i32,
            )
        });
        points.sort();
        points
    }
}
