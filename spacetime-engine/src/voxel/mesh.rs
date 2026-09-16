//! Disposable render-mesh and collision-surface extraction from voxel fields.

use avian3d::prelude::{Collider, CollisionMargin, RigidBody};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use fast_surface_nets::{
    SurfaceNetsBuffer,
    ndshape::ConstShape3u32,
    surface_nets,
};

use super::{SAMPLE_PADDING, SAMPLE_SIZE, VoxelChunk, physics};

type ChunkShape = ConstShape3u32<34, 34, 34>;

/// One extracted surface shared only as an intermediate between independently
/// owned render and physics caches.
#[derive(Component, Clone)]
pub struct VoxelRenderMesh(pub Handle<Mesh>);

impl VoxelRenderMesh {
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self(meshes.add(empty_mesh()))
    }
}

pub(crate) struct VoxelSurface {
    pub(crate) positions: Vec<[f32; 3]>,
    pub(crate) normals: Vec<[f32; 3]>,
    pub(crate) indices: Vec<u32>,
}

impl VoxelSurface {
    fn into_mesh(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
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
    debug_assert_eq!(SAMPLE_SIZE, 34);

    let mut output = SurfaceNetsBuffer::default();
    surface_nets(
        chunk.distances(),
        &ChunkShape {},
        [0; 3],
        [SAMPLE_SIZE - 1; 3],
        &mut output,
    );

    let offset = (chunk.origin() - IVec3::splat(SAMPLE_PADDING as i32)).as_vec3();
    for position in &mut output.positions {
        position[0] += offset.x;
        position[1] += offset.y;
        position[2] += offset.z;
    }

    VoxelSurface {
        positions: output.positions,
        normals: output.normals,
        indices: output.indices,
    }
}

pub(crate) fn rebuild_dirty_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: Query<(Entity, &mut VoxelChunk, &VoxelRenderMesh, Option<&Mesh3d>)>,
) {
    for (entity, mut chunk, render_mesh, visible_mesh) in &mut chunks {
        if !chunk.needs_remesh() {
            continue;
        }

        // Extract once from authoritative voxel data, then materialize two
        // independent disposable caches from that intermediate surface.
        let surface = extract_chunk_surface(&chunk);
        let collider = physics::build_chunk_collider(&chunk, &surface);
        let has_surface = !surface.positions.is_empty() && !surface.indices.is_empty();

        let mut entity_commands = commands.entity(entity);
        if has_surface {
            let Some(mut asset) = meshes.get_mut(&render_mesh.0) else {
                continue;
            };
            *asset = surface.into_mesh();
            if visible_mesh.is_none() {
                entity_commands.insert(Mesh3d(render_mesh.0.clone()));
            }
        } else if visible_mesh.is_some() {
            // Bevy 0.19's mesh allocator reports a misleading slab-allocator
            // use-after-free when an empty mesh is rendered. Keep the strong
            // asset handle as a dormant cache, but do not expose it as Mesh3d.
            entity_commands.remove::<Mesh3d>();
        }

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
