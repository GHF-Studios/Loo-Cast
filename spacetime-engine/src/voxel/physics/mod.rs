//! Disposable collision representation derived from authoritative voxel chunks.
//!
//! Physics intentionally does not read the render [`Mesh`] asset back. Both
//! render and collision representations are built from the same extracted
//! surface, so either cache can diverge later (for example lower-detail physics)
//! without changing voxel-world semantics.

use avian3d::{collision::collider::TrimeshFlags, prelude::Collider};
use bevy::prelude::*;

use super::{MATERIALIZATION_CHUNK_SIZE, mesh::VoxelSurface};

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

/// Surface Nets needs one sample of neighbor padding, so extracted surfaces from
/// adjacent chunks overlap slightly. Rendering tolerates that, physics should
/// not: duplicated triangles make contacts at chunk seams considerably noisier.
///
/// Assign each triangle to exactly one brick by its brick-local centroid. A
/// triangle may cross the brick boundary, but only one collider owns it.
pub(super) fn owned_triangles(surface: &VoxelSurface) -> Vec<[u32; 3]> {
    surface
        .indices
        .chunks_exact(3)
        .zip(&surface.triangle_materials)
        .filter_map(|(triangle, material)| {
            if !material.behavior().is_rigid() {
                return None;
            }

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
    let maximum = Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32);
    point.cmpge(Vec3::ZERO).all() && point.cmplt(maximum).all()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::voxel::{VoxelChunk, VoxelMaterialId, VoxelSample, mesh::extract_chunk_surface};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct QuantizedPoint(i32, i32, i32);

    type TriangleKey = [QuantizedPoint; 3];

    #[test]
    fn chunk_ownership_is_half_open_and_unambiguous_at_seams() {
        let size = MATERIALIZATION_CHUNK_SIZE as f32;
        assert!(owns_point(Vec3::new(size - 0.001, 1.0, 1.0)));
        assert!(!owns_point(Vec3::new(size, 1.0, 1.0)));
        assert!(owns_point(Vec3::ZERO));
    }

    #[test]
    fn collisionless_nebula_has_no_owned_collision_triangles() {
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
        assert!(owned_triangles(&surface).is_empty());
    }

    #[test]
    fn neighboring_surface_nets_seam_has_collision_ownership() {
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

        let mut visible_seam = BTreeSet::<TriangleKey>::new();
        let mut owned_seam = BTreeSet::<TriangleKey>::new();
        collect_visible_seam_triangles(&left_surface, Vec3::ZERO, size, &mut visible_seam);
        collect_visible_seam_triangles(&right_surface, right_origin, size, &mut visible_seam);
        collect_owned_seam_triangles(&left_surface, Vec3::ZERO, size, &mut owned_seam);
        collect_owned_seam_triangles(&right_surface, right_origin, size, &mut owned_seam);

        assert!(
            !visible_seam.is_empty(),
            "test sphere must cross the chunk seam"
        );
        let uncovered = visible_seam
            .difference(&owned_seam)
            .copied()
            .collect::<Vec<_>>();
        assert!(
            uncovered.is_empty(),
            "visible seam triangles without collision ownership: {uncovered:?}"
        );
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
            owned_triangles(surface),
            "voxel physics seam test",
        )
    }

    fn collect_visible_seam_triangles(
        surface: &VoxelSurface,
        world_offset: Vec3,
        size: f32,
        output: &mut BTreeSet<TriangleKey>,
    ) {
        for triangle in surface.indices.chunks_exact(3) {
            let indices = [triangle[0], triangle[1], triangle[2]];
            let centroid = triangle_centroid(surface, indices) + world_offset;
            if (centroid.x - size).abs() <= 1.25
                && centroid.y > 1.0
                && centroid.y < size - 1.0
                && centroid.z > 1.0
                && centroid.z < size - 1.0
            {
                output.insert(triangle_key(surface, indices, world_offset));
            }
        }
    }

    fn collect_owned_seam_triangles(
        surface: &VoxelSurface,
        world_offset: Vec3,
        size: f32,
        output: &mut BTreeSet<TriangleKey>,
    ) {
        for indices in owned_triangles(surface) {
            let centroid = triangle_centroid(surface, indices) + world_offset;
            if (centroid.x - size).abs() <= 1.25
                && centroid.y > 1.0
                && centroid.y < size - 1.0
                && centroid.z > 1.0
                && centroid.z < size - 1.0
            {
                output.insert(triangle_key(surface, indices, world_offset));
            }
        }
    }

    fn triangle_centroid(surface: &VoxelSurface, indices: [u32; 3]) -> Vec3 {
        let a = Vec3::from_array(surface.positions[indices[0] as usize]);
        let b = Vec3::from_array(surface.positions[indices[1] as usize]);
        let c = Vec3::from_array(surface.positions[indices[2] as usize]);
        (a + b + c) / 3.0
    }

    fn triangle_key(surface: &VoxelSurface, indices: [u32; 3], world_offset: Vec3) -> TriangleKey {
        let mut points = indices.map(|index| {
            let point = Vec3::from_array(surface.positions[index as usize]) + world_offset;
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
