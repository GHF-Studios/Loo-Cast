//! Contextual editor gizmos.
//!
//! A gizmo is broader than a viewport handle: it may expose rich state, drawing,
//! interaction and actions. This module contains the first concrete gizmo: a
//! unified Transform gizmo whose translate, rotate and scale affordances are all
//! visible at once. Visibility and edit authority are deliberately separate.

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::{
    input_focus::{InputFocus, InputFocusSet},
    view::{PrimaryGameView, PrimaryViewPresentation, ViewportSpace},
};

use super::{
    DeveloperFocus, DeveloperSet, DrawDepth, InspectAccess, InspectEditRequest, InspectField,
    InspectFieldId, InspectNumberInput, InspectSection, InspectSectionId, InspectValue,
    InspectionFrame, StructureFrame, StructureItem, StructureItemId, StructureSelection,
    WorldDrawBatch, WorldDrawFrame,
};

const INPUT_FOCUS_OWNER: &str = "editor_gizmo";
pub(in crate::devtools) const TRANSFORM_SECTION: InspectSectionId = InspectSectionId("transform");
pub(in crate::devtools) const TRANSFORM_STRUCTURE: StructureItemId = StructureItemId("core.transform");
pub(in crate::devtools) const TRANSLATION_FIELD: InspectFieldId = InspectFieldId("transform.translation");
pub(in crate::devtools) const ROTATION_FIELD: InspectFieldId = InspectFieldId("transform.rotation");
pub(in crate::devtools) const SCALE_FIELD: InspectFieldId = InspectFieldId("transform.scale");
const HANDLE_PICK_PIXELS: f32 = 9.0;
const RING_SEGMENTS: usize = 40;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EditorTransformSpace {
    #[default]
    World,
    Local,
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct EditorTransformGizmoSettings {
    transform_space: EditorTransformSpace,
}

impl EditorTransformGizmoSettings {
    pub fn transform_space(&self) -> EditorTransformSpace {
        self.transform_space
    }

    pub fn set_transform_space(&mut self, space: EditorTransformSpace) {
        self.transform_space = space;
    }
}

/// Explicit permission for the generic gizmo to mutate this runtime Transform.
///
/// Mere presence of [`Transform`] grants observation, never authority. Generated,
/// simulated, asset-authored or otherwise derived transforms should instead gain
/// domain-specific authoring adapters that commit to their real source of truth.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct EditorTransformWritable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransformOperation {
    Translate,
    Rotate,
    Scale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransformAxis {
    X,
    Y,
    Z,
}

impl TransformAxis {
    const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];

    fn vector(self) -> Vec3 {
        match self {
            Self::X => Vec3::X,
            Self::Y => Vec3::Y,
            Self::Z => Vec3::Z,
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TransformHandle {
    operation: TransformOperation,
    axis: TransformAxis,
}

#[derive(Debug, Clone)]
struct TransformDrag {
    entity: Entity,
    handle: TransformHandle,
    start_transform: Transform,
    start_cursor: Vec2,
    origin_screen: Vec2,
    axis_screen: Vec2,
    pixels_per_world: f32,
    axis_pixels: f32,
    world_axis: Vec3,
    space: EditorTransformSpace,
}

#[derive(Resource, Debug, Default)]
struct TransformGizmoInteraction {
    hovered: Option<TransformHandle>,
    drag: Option<TransformDrag>,
}

impl TransformGizmoInteraction {
    fn active(&self) -> bool {
        self.drag.is_some()
    }
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<InputFocus>()
        .init_resource::<PrimaryViewPresentation>()
        .init_resource::<EditorTransformGizmoSettings>()
        .init_resource::<TransformGizmoInteraction>()
        .add_systems(
            PreUpdate,
            claim_gizmo_input.before(InputFocusSet::Resolve),
        )
        // Inspection UI writes proposals after gameplay update; commit them in
        // the next PreUpdate, still safely before Bevy's normal PostUpdate
        // transform propagation. Viewport dragging remains immediate in Update.
        .add_systems(PreUpdate, apply_transform_inspection_edits)
        .add_systems(Update, update_transform_gizmo)
        .add_systems(
            PostUpdate,
            select_from_primary_view.in_set(DeveloperSet::Interact),
        )
        .add_systems(
            PostUpdate,
            collect_transform_structure.in_set(DeveloperSet::CollectStructure),
        )
        .add_systems(
            PostUpdate,
            collect_transform_inspection.in_set(DeveloperSet::CollectInspection),
        )
        .add_systems(
            PostUpdate,
            collect_transform_gizmo.in_set(DeveloperSet::CollectWorldDraw),
        );
}

fn claim_gizmo_input(
    presentation: Res<PrimaryViewPresentation>,
    state: Res<TransformGizmoInteraction>,
    mut input_focus: ResMut<InputFocus>,
) {
    input_focus.set_modal_claim(
        INPUT_FOCUS_OWNER,
        presentation.is_embedded() && state.active(),
    );
}

fn collect_transform_structure(
    focus: Res<DeveloperFocus>,
    transforms: Query<(), With<Transform>>,
    mut frame: ResMut<StructureFrame>,
) {
    let Some(target) = focus.current() else {
        return;
    };
    if !transforms.contains(target.spatial_entity) {
        return;
    }

    frame.submit(
        StructureItem::new(TRANSFORM_STRUCTURE, "Transform", 10)
            .detail("position, orientation and scale"),
    );
}

fn collect_transform_inspection(
    focus: Res<DeveloperFocus>,
    transforms: Query<&Transform>,
    writable: Query<(), With<EditorTransformWritable>>,
    parented: Query<(), With<ChildOf>>,
    mut frame: ResMut<InspectionFrame>,
) {
    let Some(target) = focus.current() else {
        return;
    };
    let Ok(transform) = transforms.get(target.spatial_entity) else {
        return;
    };

    let access = if writable.contains(target.spatial_entity)
        && !parented.contains(target.spatial_entity)
    {
        InspectAccess::Direct
    } else {
        InspectAccess::ReadOnly
    };
    let (rx, ry, rz) = transform.rotation.to_euler(EulerRot::XYZ);

    let translation = InspectField::new("Translation", InspectValue::Vec3(transform.translation))
        .number_input(InspectNumberInput::speed(0.02));
    let rotation = InspectField::new(
        "Rotation (degrees)",
        InspectValue::Vec3(Vec3::new(rx.to_degrees(), ry.to_degrees(), rz.to_degrees())),
    )
    .number_input(InspectNumberInput::speed(0.2));
    let scale = InspectField::new("Scale", InspectValue::Vec3(transform.scale))
        .number_input(InspectNumberInput::speed(0.01));
    let (translation, rotation, scale) = if access.editable() {
        (
            translation.editable(TRANSLATION_FIELD, access),
            rotation.editable(ROTATION_FIELD, access),
            scale.editable(SCALE_FIELD, access),
        )
    } else {
        (translation, rotation, scale)
    };

    frame.submit(
        InspectSection::new(TRANSFORM_SECTION, "Transform", 10)
            .for_structure(TRANSFORM_STRUCTURE)
            .contextual_gizmo()
            .field(translation)
            .field(rotation)
            .field(scale),
    );
}

fn apply_transform_inspection_edits(
    mut requests: MessageReader<InspectEditRequest>,
    parented: Query<(), With<ChildOf>>,
    mut transforms: Query<&mut Transform, With<EditorTransformWritable>>,
) {
    for request in requests.read() {
        if request.section != TRANSFORM_SECTION || parented.contains(request.target.spatial_entity) {
            continue;
        }
        let Ok(mut transform) = transforms.get_mut(request.target.spatial_entity) else {
            continue;
        };
        let InspectValue::Vec3(value) = request.value else {
            continue;
        };

        match request.field {
            TRANSLATION_FIELD => transform.translation = value,
            ROTATION_FIELD => {
                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    value.x.to_radians(),
                    value.y.to_radians(),
                    value.z.to_radians(),
                );
            }
            SCALE_FIELD => transform.scale = value,
            _ => {}
        }
    }
}

fn update_transform_gizmo(
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
    let (axis_screen, pixels_per_world, axis_pixels) = if handle.operation == TransformOperation::Rotate {
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

fn collect_transform_gizmo(
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
    let editable = writable.contains(target.spatial_entity) && !parented.contains(target.spatial_entity);
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

fn select_from_primary_view(
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

fn transform_context_visible(
    target: super::FocusTarget,
    structure: &StructureSelection,
) -> bool {
    structure
        .item_for(target)
        .map_or(true, |item| item == TRANSFORM_STRUCTURE)
}

fn hit_test(
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
            consider_handle(&mut best, cursor.distance(point), HANDLE_PICK_PIXELS + 2.0, scale_handle);
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
        consider_handle(
            &mut best,
            ring_distance,
            HANDLE_PICK_PIXELS,
            rotate_handle,
        );
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

fn project(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    point: Vec3,
) -> Option<Vec2> {
    camera.world_to_viewport(camera_transform, point).ok()
}

fn gizmo_world_size(camera_transform: &GlobalTransform, origin: Vec3) -> f32 {
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

fn handle_world_axis(
    handle: TransformHandle,
    space: EditorTransformSpace,
    rotation: Quat,
) -> Vec3 {
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

fn ring_basis(
    axis: TransformAxis,
    space: EditorTransformSpace,
    rotation: Quat,
) -> (Vec3, Vec3) {
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

fn handle_color(axis: TransformAxis, highlighted: bool, editable: bool) -> Color {
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
