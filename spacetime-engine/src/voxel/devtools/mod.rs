//! Voxel realization developer visualization.

use bevy::prelude::*;

use crate::{
    devtools::{DeveloperSet, DeveloperTools, DrawDepth, WorldDrawBatch, WorldDrawFrame},
    spatial::{SPATIAL_DEMAND_VISUALIZATION, UsfPrimaryInteractionSlice, UsfScaleLayer, UsfSpatialFrame},
};

use super::{MATERIALIZATION_CHUNK_SIZE, VoxelWorld};

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        PostUpdate,
        collect_voxel_materialization_world_draw.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_voxel_materialization_world_draw(
    tools: Res<DeveloperTools>,
    spatial_frame: Res<UsfSpatialFrame>,
    interaction: Res<UsfPrimaryInteractionSlice>,
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
        // This overlay is drawn in the local physical view, not in all 71
        // numerical charts superimposed on one another.
        if layer.scale() != interaction.scale() {
            continue;
        }
        for address in world.materializations().active_addresses() {
            let Ok(translation) = address.origin().relative_at_scale_bounded(
                spatial_frame.origin(), layer.scale(), 16_384.0,
            )
            else {
                continue;
            };
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
