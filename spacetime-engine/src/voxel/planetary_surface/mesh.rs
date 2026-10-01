//! Cubed-sphere mesh construction for regional planetary presentation.
//!
//! Vertices are body-local and sampled from the same `CelestialVoxelField`
//! surface resolver used by dense celestial voxels. Runtime translation and
//! orientation remain derived presentation state.

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    spatial::SpatialScale,
    voxel::CelestialVoxelField,
};

use super::PlanetarySurfacePatchId;

pub(super) const PATCH_GRID_RESOLUTION: u32 = 8;

/// Builds one bounded body-local cubed-sphere patch.
///
/// `sample_scale` selects semantic terrain detail. It is not patch-subdivision
/// level and therefore does not couple representation refinement to USF Scale.
pub(super) fn build_planetary_surface_patch(
    field: CelestialVoxelField,
    patch: PlanetarySurfacePatchId,
    sample_scale: SpatialScale,
) -> Option<Mesh> {
    let side = PATCH_GRID_RESOLUTION + 1;
    let vertex_count = (side * side) as usize;
    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);

    for y in 0..=PATCH_GRID_RESOLUTION {
        let v = y as f32 / PATCH_GRID_RESOLUTION as f32;
        for x in 0..=PATCH_GRID_RESOLUTION {
            let u = x as f32 / PATCH_GRID_RESOLUTION as f32;
            let direction = patch.direction_at(u, v);
            let position =
                sample_patch_vertex_native(field, direction, sample_scale)?;
            positions.push(position.to_array());
            normals.push(direction.to_array());
            uvs.push([u, v]);
        }
    }

    let mut indices =
        Vec::with_capacity((PATCH_GRID_RESOLUTION * PATCH_GRID_RESOLUTION * 6) as usize);
    for y in 0..PATCH_GRID_RESOLUTION {
        for x in 0..PATCH_GRID_RESOLUTION {
            let a = y * side + x;
            let b = a + 1;
            let c = a + side;
            let d = c + 1;
            indices.extend_from_slice(&[a, b, c, b, d, c]);
        }
    }

    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}

pub(super) fn sample_patch_vertex_native(
    field: CelestialVoxelField,
    local_direction: Vec3,
    sample_scale: SpatialScale,
) -> Option<Vec3> {
    let local_metres = field
        .surface_local_metres(local_direction, sample_scale)
        .ok()?;
    let native = local_metres / sample_scale.metres_per_native();
    let native = Vec3::new(native.x as f32, native.y as f32, native.z as f32);
    native.is_finite().then_some(native)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        spatial::{UsfPosition, UsfSemanticFrame},
        voxel::CelestialBodyProfile,
    };

    #[test]
    fn patch_vertex_is_exactly_the_shared_semantic_surface_in_body_local_space() {
        let origin = UsfPosition::zero(SpatialScale::MIN);
        let frame = UsfSemanticFrame::identity();
        let scale = SpatialScale::new(4).unwrap();
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );
        let direction = Vec3::new(0.41, 0.77, -0.49).normalize();

        let local_native =
            sample_patch_vertex_native(field, direction, scale).unwrap();
        let reconstructed = frame
            .local_metres_to_world(
                origin,
                DVec3::new(
                    f64::from(local_native.x),
                    f64::from(local_native.y),
                    f64::from(local_native.z),
                ) * scale.metres_per_native(),
            )
            .unwrap();
        let semantic = field
            .surface_position(&origin, frame, direction, scale)
            .unwrap();

        let delta = reconstructed
            .relative_at_scale_bounded_f64(&semantic, scale, 0.01)
            .unwrap();
        assert!(
            delta.length() < 1.0e-3,
            "regional patch vertex diverged from semantic surface: {delta:?}"
        );

        let dense_resolver = field.realization(origin, frame, scale);
        assert_eq!(
            dense_resolver.surface_position(direction).unwrap(),
            semantic,
            "dense and regional surface resolvers must share terrain truth",
        );
    }
}
