//! Transform-gizmo input, dragging and viewport selection.

use super::*;

pub(super) fn claim_gizmo_input(
    presentation: Res<PrimaryViewPresentation>,
    state: Res<TransformGizmoInteraction>,
    mut input_focus: ResMut<InputFocus>,
) {
    input_focus.set_modal_claim(
        INPUT_FOCUS_OWNER,
        presentation.is_embedded() && state.active(),
    );
}

pub(super) fn update_transform_gizmo(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    presentation: Res<PrimaryViewPresentation>,
    window: Single<(&Window, &CursorOptions), With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<PrimaryGameView>>,
    focus: Res<DeveloperFocus>,
    structure: Res<StructureSelection>,
    settings: Res<EditorTransformGizmoSettings>,
    globals: Query<&GlobalTransform>,
    parented: Query<(), With<ChildOf>>,
    mut transforms: Query<&mut Transform, With<EditorTransformWritable>>,
    mut state: ResMut<TransformGizmoInteraction>,
) {
    if !presentation.is_embedded() {
        state.hovered = None;
        state.drag = None;
        return;
    }

    let (window, cursor_options) = window.into_inner();
    let (camera, camera_transform) = camera.into_inner();
    let Some(cursor) = window.cursor_position() else {
        state.hovered = None;
        return;
    };
    if cursor_options.grab_mode != CursorGrabMode::None
        || !ViewportSpace::new(camera).contains_target_position(cursor)
    {
        state.hovered = None;
        return;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        if let Some(drag) = state.drag.take()
            && let Ok(mut transform) = transforms.get_mut(drag.entity)
        {
            *transform = drag.start_transform;
        }
        return;
    }

    if mouse.just_released(MouseButton::Left) {
        state.drag = None;
    }

    if let Some(drag) = state.drag.clone() {
        if !mouse.pressed(MouseButton::Left) {
            state.drag = None;
            return;
        }
        let Ok(mut transform) = transforms.get_mut(drag.entity) else {
            state.drag = None;
            return;
        };
        apply_drag(&mut transform, &drag, cursor);
        state.hovered = Some(drag.handle);
        return;
    }

    let Some(target) = focus.current() else {
        state.hovered = None;
        return;
    };
    if !transform_context_visible(target, &structure) {
        state.hovered = None;
        return;
    }
    let Ok(global) = globals.get(target.spatial_entity) else {
        state.hovered = None;
        return;
    };

    state.hovered = hit_test(
        camera,
        camera_transform,
        global,
        settings.transform_space(),
        cursor,
    );

    if !mouse.just_pressed(MouseButton::Left)
        || state.hovered.is_none()
        || parented.contains(target.spatial_entity)
    {
        return;
    }
    let Ok(transform) = transforms.get_mut(target.spatial_entity) else {
        return;
    };
    let handle = state.hovered.unwrap();
    let transform_snapshot = (*transform).clone();
    drop(transform);

    let world = global.compute_transform();
    let origin = world.translation;
    let size = gizmo_world_size(camera_transform, origin);
    let world_axis = handle_world_axis(handle, settings.transform_space(), world.rotation);
    let Some(origin_screen) = project(camera, camera_transform, origin) else {
        return;
    };
    let (axis_screen, pixels_per_world, axis_pixels) = if handle.operation
        == TransformOperation::Rotate
    {
        // Rotation uses cursor angle around the projected origin. A rotation
        // axis pointing toward the camera has almost no screen projection but
        // its ring is maximally useful, so it must not fail drag initialization.
        (Vec2::ZERO, 1.0, 1.0)
    } else {
        let Some(axis_end) = project(camera, camera_transform, origin + world_axis * size) else {
            return;
        };
        let axis_delta = axis_end - origin_screen;
        let axis_pixels = axis_delta.length();
        if axis_pixels <= 1.0 {
            return;
        }
        (
            axis_delta / axis_pixels,
            axis_pixels / size.max(1.0e-5),
            axis_pixels,
        )
    };

    state.drag = Some(TransformDrag {
        entity: target.spatial_entity,
        handle,
        start_transform: transform_snapshot,
        start_cursor: cursor,
        origin_screen,
        axis_screen,
        pixels_per_world,
        axis_pixels,
        world_axis,
        space: settings.transform_space(),
    });
}

fn apply_drag(transform: &mut Transform, drag: &TransformDrag, cursor: Vec2) {
    *transform = drag.start_transform.clone();

    match drag.handle.operation {
        TransformOperation::Translate => {
            let pixels = (cursor - drag.start_cursor).dot(drag.axis_screen);
            let distance = pixels / drag.pixels_per_world.max(1.0e-5);
            transform.translation += drag.world_axis * distance;
        }
        TransformOperation::Rotate => {
            let start = (drag.start_cursor - drag.origin_screen).normalize_or_zero();
            let current = (cursor - drag.origin_screen).normalize_or_zero();
            if start == Vec2::ZERO || current == Vec2::ZERO {
                return;
            }
            let cross = start.x * current.y - start.y * current.x;
            let angle = -cross.atan2(start.dot(current));
            let local_axis = drag.handle.axis.vector();
            transform.rotation = match drag.space {
                EditorTransformSpace::World => {
                    Quat::from_axis_angle(drag.world_axis, angle) * drag.start_transform.rotation
                }
                EditorTransformSpace::Local => {
                    drag.start_transform.rotation * Quat::from_axis_angle(local_axis, angle)
                }
            };
        }
        TransformOperation::Scale => {
            let pixels = (cursor - drag.start_cursor).dot(drag.axis_screen);
            let factor = (1.0 + pixels / drag.axis_pixels.max(1.0)).max(0.01);
            let start = drag.start_transform.scale;
            match drag.handle.axis {
                TransformAxis::X => transform.scale.x = start.x * factor,
                TransformAxis::Y => transform.scale.y = start.y * factor,
                TransformAxis::Z => transform.scale.z = start.z * factor,
            }
        }
    }
}

pub(super) fn select_from_primary_view(
    mouse: Res<ButtonInput<MouseButton>>,
    presentation: Res<PrimaryViewPresentation>,
    window: Single<(&Window, &CursorOptions), With<PrimaryWindow>>,
    camera: Single<&Camera, With<PrimaryGameView>>,
    mut focus: ResMut<DeveloperFocus>,
    gizmo: Res<TransformGizmoInteraction>,
    mut structure: ResMut<StructureSelection>,
) {
    if !presentation.is_embedded() || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let (window, cursor) = window.into_inner();
    if cursor.grab_mode != CursorGrabMode::None || gizmo.active() || gizmo.hovered.is_some() {
        return;
    }

    let Some(position) = window.cursor_position() else {
        return;
    };
    if !ViewportSpace::new(&camera).contains_target_position(position) {
        return;
    }

    focus.clear_pin();
    if let Some(target) = focus.hovered() {
        focus.select(target);
        structure.select_entity(target);
    } else {
        focus.clear_selection();
        structure.clear();
    }
}
