//! World Draw visualization of hierarchical spatial-demand scopes.

use super::*;

pub(super) fn collect_spatial_demand_world_draw(
    tools: Res<DeveloperTools>,
    spatial_frame: Res<UsfSpatialFrame>,
    demands: Res<SpatialDemandSnapshot>,
    frame: Res<WorldDrawFrame>,
) {
    if !tools.visualization_enabled(SPATIAL_DEMAND_VISUALIZATION) {
        return;
    }

    let mut batch = WorldDrawBatch::default();
    let color = Color::srgba(0.20, 0.82, 1.0, 0.92);

    for demand in demands.iter() {
        let Ok(center) = demand
            .center()
            .relative_native_bounded(spatial_frame.origin(), DEMAND_DEBUG_PROJECTION_BOUND)
        else {
            continue;
        };
        let half = demand.half_extent_native();
        draw_wire_box(&mut batch, center - half, center + half, color);
        batch.cross(
            Isometry3d::new(center, Quat::IDENTITY),
            0.45,
            color,
            DrawDepth::Overlay,
        );
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
