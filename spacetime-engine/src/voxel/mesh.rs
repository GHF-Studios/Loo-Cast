//! Disposable render-mesh and collision-surface extraction from voxel fields.

use avian3d::prelude::{Collider, CollisionMargin, RigidBody};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use fast_surface_nets::{SurfaceNetsBuffer, ndshape::ConstShape3u32, surface_nets};

use super::{
    VoxelChunk,
    chunk::{SAMPLE_PADDING, SAMPLE_SIZE},
    physics,
};

type ChunkShape = ConstShape3u32<SAMPLE_SIZE, SAMPLE_SIZE, SAMPLE_SIZE>;

/// One extracted surface shared only as an intermediate between independently
/// owned render and physics caches.
pub(crate) struct VoxelSurface {
    pub(crate) positions: Vec<[f32; 3]>,
    pub(crate) normals: Vec<[f32; 3]>,
    pub(crate) uvs: Vec<[f32; 2]>,
    pub(crate) tangents: Vec<[f32; 4]>,
    pub(crate) indices: Vec<u32>,
}

impl VoxelSurface {
    pub(crate) fn into_mesh(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, self.tangents)
        .with_inserted_indices(Indices::U32(self.indices))
    }
}

pub(crate) fn empty_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
}

pub(crate) fn extract_chunk_surface(chunk: &VoxelChunk) -> VoxelSurface {
    let mut output = SurfaceNetsBuffer::default();
    surface_nets(
        chunk.distances(),
        &ChunkShape {},
        [0; 3],
        [SAMPLE_SIZE - 1; 3],
        &mut output,
    );

    // Surface Nets coordinates start at the padded sample allocation. Keep the
    // derived surface local to the logical brick origin; the brick entity's
    // Transform performs runtime projection into the current local chart.
    let offset = Vec3::splat(-(SAMPLE_PADDING as f32));
    for position in &mut output.positions {
        position[0] += offset.x;
        position[1] += offset.y;
        position[2] += offset.z;
    }

    let (uvs, tangents) = surface_projection_attributes(&output.positions, &output.normals);

    VoxelSurface {
        positions: output.positions,
        normals: output.normals,
        uvs,
        tangents,
        indices: output.indices,
    }
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

pub(crate) fn rebuild_dirty_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: Query<(Entity, &mut VoxelChunk, &Mesh3d)>,
) {
    for (entity, mut chunk, mesh) in &mut chunks {
        if !chunk.needs_remesh() {
            continue;
        }

        let Some(mut asset) = meshes.get_mut(&mesh.0) else {
            continue;
        };

        // Extract once from authoritative voxel data, then materialize two
        // independent disposable caches from that intermediate surface.
        let surface = extract_chunk_surface(&chunk);
        let collider = physics::build_chunk_collider(&chunk, &surface);
        *asset = surface.into_mesh();

        let mut entity_commands = commands.entity(entity);
        if let Some(collider) = collider {
            entity_commands.insert((
                RigidBody::Static,
                collider,
                CollisionMargin(physics::VOXEL_COLLISION_MARGIN),
            ));
        } else {
            entity_commands.remove::<Collider>();
        }

        chunk.mark_meshed();
    }
}
