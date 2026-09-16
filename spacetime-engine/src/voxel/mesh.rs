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
