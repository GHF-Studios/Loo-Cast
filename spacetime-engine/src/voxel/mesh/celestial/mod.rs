//! Bounded whole-body presentation sampled from the celestial voxel baseline.
//!
//! This is a finite triangulation of the same radial field, not another sphere
//! defining an unrelated ground surface. It does not publish collision or
//! coverage, and does not yet aggregate sparse voxel edits.

use bevy::{mesh::VertexAttributeValues, prelude::*};

use crate::{spatial::SpatialScale, voxel::CelestialVoxelField};

pub(crate) fn celestial_surface_mesh(
    field: CelestialVoxelField,
    surface_scale: SpatialScale,
    mesh_scale: SpatialScale,
) -> Mesh {
    let sampler = field.realization(surface_scale);
    let native_to_mesh =
        10.0_f64.powi(i32::from(surface_scale.exponent()) - i32::from(mesh_scale.exponent()));
    // Keep approximately the existing whole-body tessellation cost. Finer
    // surface detail still belongs to demand-driven voxel realizations.
    let mut mesh = Sphere::new(1.0)
        .mesh()
        .ico(5)
        .expect("fixed celestial sphere subdivision is valid");
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    else {
        unreachable!("sphere mesh positions are Float32x3");
    };
    for position in positions {
        let direction = Vec3::from_array(*position).normalize();
        let radius = f64::from(sampler.surface_radius_native(direction)) * native_to_mesh;
        *position = (direction * radius as f32).to_array();
    }
    mesh.compute_smooth_normals();
    mesh
}
