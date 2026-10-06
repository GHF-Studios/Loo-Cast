//! World-draw presentation of Transform gizmo handles.

use super::*;

pub(super) fn collect_transform_gizmo(
    presentation: Res<PrimaryViewPresentation>,
    focus: Res<DeveloperFocus>,
    structure: Res<StructureSelection>,
    settings: Res<EditorTransformGizmoSettings>,
    state: Res<TransformGizmoInteraction>,
    camera: Single<&GlobalTransform, With<PrimaryGameView>>,
    globals: Query<&GlobalTransform>,
    writable: Query<(), With<EditorTransformWritable>>,
    parented: Query<(), With<ChildOf>>,
    frame: Res<WorldDrawFrame>,
) {
    if !presentation.is_embedded() {
        return;
    }
    let Some(target) = focus.current() else {
        return;
    };
    if !transform_context_visible(target, &structure) {
        return;
    }
    let Ok(global) = globals.get(target.spatial_entity) else {
        return;
    };

    let camera_transform = camera.into_inner();
    let transform = global.compute_transform();
    let origin = transform.translation;
    let size = gizmo_world_size(camera_transform, origin);
    let editable =
        writable.contains(target.spatial_entity) && !parented.contains(target.spatial_entity);
    let mut batch = WorldDrawBatch::default();

    for axis in TransformAxis::ALL {
        let geometry = AxisHandleGeometry::new(
            axis,
            origin,
            size,
            transform.rotation,
            settings.transform_space(),
        );
        draw_axis_handles(
            &mut batch,
            origin,
            size,
            axis,
            &geometry,
            state.hovered,
            editable,
        );
    }

    batch.cross(
        Isometry3d::new(origin, Quat::IDENTITY),
        size * 0.07,
        if editable {
            Color::srgb(0.95, 0.95, 0.95)
        } else {
            Color::srgb(0.45, 0.45, 0.45)
        },
        DrawDepth::Overlay,
    );
    frame.submit(batch);
}

/// Draw from the same handle geometry used for viewport hit testing.
fn draw_axis_handles(
    batch: &mut WorldDrawBatch,
    origin: Vec3,
    size: f32,
    axis: TransformAxis,
    geometry: &AxisHandleGeometry,
    hovered: Option<TransformHandle>,
    editable: bool,
) {
    let translate = TransformHandle {
        operation: TransformOperation::Translate,
        axis,
    };
    let scale = TransformHandle {
        operation: TransformOperation::Scale,
        axis,
    };
    let rotate = TransformHandle {
        operation: TransformOperation::Rotate,
        axis,
    };
    batch.arrow(
        geometry.translate_start,
        geometry.translate_end,
        handle_color(axis, hovered == Some(translate), editable),
        DrawDepth::Overlay,
    );
    batch.line(
        origin,
        geometry.scale_end,
        handle_color(axis, hovered == Some(scale), editable),
        DrawDepth::Overlay,
    );
    batch.cross(
        Isometry3d::new(geometry.scale_end, Quat::IDENTITY),
        size * 0.055,
        handle_color(axis, hovered == Some(scale), editable),
        DrawDepth::Overlay,
    );

    let ring_color = handle_color(axis, hovered == Some(rotate), editable);
    let mut ring_points = geometry.ring_points();
    let mut previous = ring_points
        .next()
        .expect("ring geometry has its first point");
    for point in ring_points {
        batch.line(previous, point, ring_color, DrawDepth::Overlay);
        previous = point;
    }
}
