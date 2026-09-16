//! Disposable render-mesh extraction from authoritative voxel fields.

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

use super::{SAMPLE_PADDING, SAMPLE_SIZE, VoxelChunk};

type ChunkShape = ConstShape3u32<34, 34, 34>;

pub(crate) fn empty_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
}

pub(crate) fn build_chunk_mesh(chunk: &VoxelChunk) -> Mesh {
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

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, output.positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, output.normals)
    .with_inserted_indices(Indices::U32(output.indices))
}

pub(crate) fn remesh_dirty_chunks(
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: Query<(&mut VoxelChunk, &Mesh3d)>,
) {
    for (mut chunk, mesh) in &mut chunks {
        if !chunk.needs_remesh() {
            continue;
        }

        let Some(mut asset) = meshes.get_mut(&mesh.0) else {
            continue;
        };

        *asset = build_chunk_mesh(&chunk);
        chunk.mark_meshed();
    }
}
