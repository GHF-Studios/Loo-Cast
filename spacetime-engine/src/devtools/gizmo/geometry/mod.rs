//! Projection, axis mapping and handle hit-testing for Transform gizmos.

use super::*;

pub(super) fn transform_context_visible(target: super::FocusTarget, structure: &StructureSelection) -> bool {
    structure
        .item_for(target)
        .map_or(true, |item| item == TRANSFORM_STRUCTURE)
}

pub(super) fn hit_test(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    global: &GlobalTransform,
    space: EditorTransformSpace,
    cursor: Vec2,
) -> Option<TransformHandle> {
    let transform = global.compute_transform();
    let origin = transform.translation;
    let size = gizmo_world_size(camera_transform, origin);
    let origin_screen = project(camera, camera_transform, origin)?;
    let mut best: Option<(f32, TransformHandle)> = None;

    for axis in TransformAxis::ALL {
        let scale_handle = TransformHandle {
            operation: TransformOperation::Scale,
            axis,
        };
        let scale_direction = handle_world_axis(scale_handle, space, transform.rotation);
        if let Some(point) = project(
            camera,
            camera_transform,
            origin + scale_direction * size * 0.58,
        ) {
            consider_handle(
                &mut best,
                cursor.distance(point),
                HANDLE_PICK_PIXELS + 2.0,
                scale_handle,
            );
        }

        let translate_handle = TransformHandle {
            operation: TransformOperation::Translate,
            axis,
        };
        let translate_direction = handle_world_axis(translate_handle, space, transform.rotation);
        if let (Some(a), Some(b)) = (
            project(
                camera,
                camera_transform,
                origin + translate_direction * size * 0.68,
            ),
            project(
                camera,
                camera_transform,
                origin + translate_direction * size * 1.12,
            ),
        ) {
            consider_handle(
                &mut best,
                point_segment_distance(cursor, a, b),
                HANDLE_PICK_PIXELS,
                translate_handle,
            );
        }

        let rotate_handle = TransformHandle {
            operation: TransformOperation::Rotate,
            axis,
        };
        let (basis_a, basis_b) = ring_basis(axis, space, transform.rotation);
        let radius = size * 0.82;
        let mut previous = project(camera, camera_transform, origin + basis_a * radius);
        let mut ring_distance = f32::INFINITY;
        for segment in 1..=RING_SEGMENTS {
            let angle = std::f32::consts::TAU * segment as f32 / RING_SEGMENTS as f32;
            let point = origin + (basis_a * angle.cos() + basis_b * angle.sin()) * radius;
            let current = project(camera, camera_transform, point);
            if let (Some(a), Some(b)) = (previous, current) {
                ring_distance = ring_distance.min(point_segment_distance(cursor, a, b));
            }
            previous = current;
        }
        consider_handle(&mut best, ring_distance, HANDLE_PICK_PIXELS, rotate_handle);
    }

    // Avoid selecting an almost edge-on gizmo merely because its projected
    // geometry collapsed onto the origin.
    best.filter(|(distance, _)| *distance <= HANDLE_PICK_PIXELS + 2.0)
        .map(|(_, handle)| handle)
        .filter(|_| origin_screen.is_finite())
}

fn consider_handle(
    best: &mut Option<(f32, TransformHandle)>,
    distance: f32,
    threshold: f32,
    handle: TransformHandle,
) {
    if !distance.is_finite() || distance > threshold {
        return;
    }
    if best
        .as_ref()
        .map_or(true, |(best_distance, _)| distance < *best_distance)
    {
        *best = Some((distance, handle));
    }
}

fn point_segment_distance(point: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let length_squared = ab.length_squared();
    if length_squared <= 1.0e-5 {
        return point.distance(a);
    }
    let t = ((point - a).dot(ab) / length_squared).clamp(0.0, 1.0);
    point.distance(a + ab * t)
}

pub(super) fn project(camera: &Camera, camera_transform: &GlobalTransform, point: Vec3) -> Option<Vec2> {
    camera.world_to_viewport(camera_transform, point).ok()
}

pub(super) fn gizmo_world_size(camera_transform: &GlobalTransform, origin: Vec3) -> f32 {
    camera_transform
        .translation()
        .distance(origin)
        .mul_add(0.12, 0.0)
        .clamp(0.25, 12.0)
}

fn world_axis(axis: TransformAxis, space: EditorTransformSpace, rotation: Quat) -> Vec3 {
    match space {
        EditorTransformSpace::World => axis.vector(),
        EditorTransformSpace::Local => rotation * axis.vector(),
    }
}

pub(super) fn handle_world_axis(handle: TransformHandle, space: EditorTransformSpace, rotation: Quat) -> Vec3 {
    // `Transform::scale` is local-axis data. A true world-space scale operation
    // on a rotated transform would need decomposition/authority semantics beyond
    // this generic direct-runtime adapter, so scale handles deliberately remain
    // local even while translation/rotation are shown in World space.
    let effective_space = if handle.operation == TransformOperation::Scale {
        EditorTransformSpace::Local
    } else {
        space
    };
    world_axis(handle.axis, effective_space, rotation)
}

pub(super) fn ring_basis(axis: TransformAxis, space: EditorTransformSpace, rotation: Quat) -> (Vec3, Vec3) {
    let (a, b) = match axis {
        TransformAxis::X => (Vec3::Y, Vec3::Z),
        TransformAxis::Y => (Vec3::Z, Vec3::X),
        TransformAxis::Z => (Vec3::X, Vec3::Y),
    };
    match space {
        EditorTransformSpace::World => (a, b),
        EditorTransformSpace::Local => (rotation * a, rotation * b),
    }
}

pub(super) fn handle_color(axis: TransformAxis, highlighted: bool, editable: bool) -> Color {
    if highlighted && editable {
        return Color::srgb(1.0, 1.0, 1.0);
    }

    let strength = if editable { 1.0 } else { 0.45 };
    match axis {
        TransformAxis::X => Color::srgb(strength, 0.12 * strength, 0.12 * strength),
        TransformAxis::Y => Color::srgb(0.12 * strength, strength, 0.18 * strength),
        TransformAxis::Z => Color::srgb(0.16 * strength, 0.42 * strength, strength),
    }
}
