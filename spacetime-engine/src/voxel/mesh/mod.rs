//! Disposable render-mesh and collision-surface extraction from voxel fields.

use bevy::prelude::{Vec2, Vec3};
use fast_surface_nets::{SurfaceNetsBuffer, ndshape::ConstShape3u32, surface_nets};

use super::{
    VoxelChunk, VoxelMaterialId,
    chunk::{SAMPLE_PADDING, SAMPLE_SIZE},
};

type ChunkShape = ConstShape3u32<SAMPLE_SIZE, SAMPLE_SIZE, SAMPLE_SIZE>;

/// One extracted surface shared only as an intermediate between independently
/// owned render and physics caches.
#[derive(Debug)]
pub(super) struct VoxelSurface {
    pub(super) positions: Vec<[f32; 3]>,
    pub(super) normals: Vec<[f32; 3]>,
    pub(super) uvs: Vec<[f32; 2]>,
    pub(super) tangents: Vec<[f32; 4]>,
    pub(super) indices: Vec<u32>,
    pub(super) vertex_materials: Vec<VoxelMaterialId>,
    pub(super) triangle_materials: Vec<VoxelMaterialId>,
}

impl VoxelSurface {
    pub(super) fn opaque_indices(&self) -> Vec<u32> {
        self.indices
            .chunks_exact(3)
            .zip(&self.triangle_materials)
            .filter(|(_, material)| !material.behavior().is_translucent())
            .flat_map(|(triangle, _)| triangle.iter().copied())
            .collect()
    }

    pub(super) fn translucent_indices(&self) -> Vec<u32> {
        self.indices
            .chunks_exact(3)
            .zip(&self.triangle_materials)
            .filter(|(_, material)| material.behavior().is_translucent())
            .flat_map(|(triangle, _)| triangle.iter().copied())
            .collect()
    }

    pub(super) fn has_rigid_triangles(&self) -> bool {
        self.triangle_materials
            .iter()
            .any(|material| material.behavior().is_rigid())
    }
}

pub(super) fn extract_chunk_surface(chunk: &VoxelChunk) -> VoxelSurface {
    let mut output = SurfaceNetsBuffer::default();
    surface_nets(
        chunk.distances(),
        &ChunkShape {},
        [0; 3],
        [SAMPLE_SIZE - 1; 3],
        &mut output,
    );

    // Surface Nets coordinates start at the padded sample allocation. Keep the
    // derived surface local to the logical brick origin. Rendering projects that
    // canonical brick origin into the active view; the scale-local physics root
    // independently projects the same address into its Avian slice.
    let offset = Vec3::splat(-(SAMPLE_PADDING as f32));
    for position in &mut output.positions {
        position[0] += offset.x;
        position[1] += offset.y;
        position[2] += offset.z;
    }

    let (uvs, tangents) = surface_projection_attributes(&output.positions, &output.normals);
    let vertex_materials = output
        .positions
        .iter()
        .map(|position| surface_material_near(chunk, Vec3::from_array(*position)))
        .collect::<Vec<_>>();
    let triangle_materials = output
        .indices
        .chunks_exact(3)
        .map(|triangle| {
            let a = Vec3::from_array(output.positions[triangle[0] as usize]);
            let b = Vec3::from_array(output.positions[triangle[1] as usize]);
            let c = Vec3::from_array(output.positions[triangle[2] as usize]);
            surface_material_near(chunk, (a + b + c) / 3.0)
        })
        .collect();

    VoxelSurface {
        positions: output.positions,
        normals: output.normals,
        uvs,
        tangents,
        indices: output.indices,
        vertex_materials,
        triangle_materials,
    }
}

fn surface_material_near(chunk: &VoxelChunk, point: Vec3) -> VoxelMaterialId {
    let center = point.floor().as_ivec3();
    let mut best = None::<(f32, VoxelMaterialId)>;

    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                let Some(sample) = chunk.sample(center + bevy::prelude::IVec3::new(x, y, z)) else {
                    continue;
                };
                if !sample.distance.is_solid() || sample.material == VoxelMaterialId::VOID {
                    continue;
                }
                let score = sample.distance.0.abs();
                if best.is_none_or(|(current, _)| score < current) {
                    best = Some((score, sample.material));
                }
            }
        }
    }

    best.map_or(VoxelMaterialId::ROCK, |(_, material)| material)
}

/// Immediate world-aligned texture projection for arbitrary Surface Nets geometry.
///
/// This is intentionally simpler than the eventual shader-side triplanar path,
/// but unlike the old attribute-less mesh it makes generated PBR textures usable
/// on voxel terrain now. A 2-metre repeat divides the decimal 10-metre base
/// materialization extent exactly, keeping adjacent chunk phases aligned.
fn surface_projection_attributes(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
) -> (Vec<[f32; 2]>, Vec<[f32; 4]>) {
    const TILES_PER_METER: f32 = 0.5;

    let mut uvs = Vec::with_capacity(positions.len());
    let mut tangents = Vec::with_capacity(positions.len());
    for (position, normal) in positions.iter().zip(normals) {
        let position = Vec3::from_array(*position);
        let normal = Vec3::from_array(*normal).normalize_or_zero();
        let axis = normal.abs();

        let (uv, tangent_axis, handedness) = if axis.x >= axis.y && axis.x >= axis.z {
            (
                Vec2::new(position.z, position.y),
                Vec3::Z,
                if normal.x >= 0.0 { -1.0 } else { 1.0 },
            )
        } else if axis.y >= axis.z {
            (
                Vec2::new(position.x, position.z),
                Vec3::X,
                if normal.y >= 0.0 { -1.0 } else { 1.0 },
            )
        } else {
            (
                Vec2::new(position.x, position.y),
                Vec3::X,
                if normal.z >= 0.0 { 1.0 } else { -1.0 },
            )
        };

        let tangent = (tangent_axis - normal * normal.dot(tangent_axis)).normalize_or_zero();
        uvs.push((uv * TILES_PER_METER).to_array());
        tangents.push([tangent.x, tangent.y, tangent.z, handedness]);
    }

    (uvs, tangents)
}
