//! Voxel realization developer visualization.

use bevy::prelude::*;

use crate::{
    devtools::{DeveloperSet, DeveloperTools, DrawDepth, WorldDrawBatch, WorldDrawFrame},
    spatial::{SPATIAL_DEMAND_VISUALIZATION, UsfScaleLayer, UsfScaleLayerFrames},
};

use super::{MATERIALIZATION_CHUNK_SIZE, VoxelQueryPosition, VoxelWorld};

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        PostUpdate,
        collect_voxel_materialization_world_draw.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_voxel_materialization_world_draw(
    tools: Res<DeveloperTools>,
    frames: Res<UsfScaleLayerFrames>,
    worlds: Query<(&VoxelWorld, &UsfScaleLayer)>,
    frame: Res<WorldDrawFrame>,
) {
    if !tools.visualization_enabled(SPATIAL_DEMAND_VISUALIZATION) {
        return;
    }

    let mut batch = WorldDrawBatch::default();
    let size = MATERIALIZATION_CHUNK_SIZE as f32;
    let extent = Vec3::splat(size);
    let color = Color::srgba(0.35, 1.0, 0.38, 0.82);

    for (world, layer) in &worlds {
        let world_origin = VoxelQueryPosition::new(*world.origin());
        for address in world.materializations().active_addresses() {
            let Ok(relative) = address
                .query_origin()
                .relative_to(world_origin, 1_000_000.0)
            else {
                continue;
            };
            let absolute =
                bevy::math::DVec3::new(relative.x as f64, relative.y as f64, relative.z as f64);
            let translation = frames.runtime_from_absolute(layer.scale(), absolute);
            draw_wire_box(&mut batch, translation, translation + extent, color);
        }
    }

    frame.submit(batch);
}

fn draw_wire_box(batch: &mut WorldDrawBatch, min: Vec3, max: Vec3, color: Color) {
    let corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(min.x, max.y, max.z),
        Vec3::new(max.x, max.y, max.z),
    ];

    for (a, b) in [
        (0, 1),
        (0, 2),
        (1, 3),
        (2, 3),
        (4, 5),
        (4, 6),
        (5, 7),
        (6, 7),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ] {
        batch.line(corners[a], corners[b], color, DrawDepth::Overlay);
    }
}
