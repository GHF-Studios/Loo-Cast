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
        let translate_direction =
            handle_world_axis(translate, settings.transform_space(), transform.rotation);
        let scale_direction =
            handle_world_axis(scale, settings.transform_space(), transform.rotation);

        batch.arrow(
            origin + translate_direction * size * 0.68,
            origin + translate_direction * size * 1.12,
            handle_color(axis, state.hovered == Some(translate), editable),
            DrawDepth::Overlay,
        );
        batch.line(
            origin,
            origin + scale_direction * size * 0.58,
            handle_color(axis, state.hovered == Some(scale), editable),
            DrawDepth::Overlay,
        );
        batch.cross(
            Isometry3d::new(origin + scale_direction * size * 0.58, Quat::IDENTITY),
            size * 0.055,
            handle_color(axis, state.hovered == Some(scale), editable),
            DrawDepth::Overlay,
        );

        let ring_color = handle_color(axis, state.hovered == Some(rotate), editable);
        let (basis_a, basis_b) = ring_basis(axis, settings.transform_space(), transform.rotation);
        let radius = size * 0.82;
        let mut previous = origin + basis_a * radius;
        for segment in 1..=RING_SEGMENTS {
            let angle = std::f32::consts::TAU * segment as f32 / RING_SEGMENTS as f32;
            let point = origin + (basis_a * angle.cos() + basis_b * angle.sin()) * radius;
            batch.line(previous, point, ring_color, DrawDepth::Overlay);
            previous = point;
        }
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
