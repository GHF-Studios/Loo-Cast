use bevy::prelude::*;

use crate::config::statics::CONFIG;
use crate::core::components::Meta;
use crate::picking::constants::META_MOUSE_POINTER_ID;
use crate::render::resources::{PrimaryWindowUiState, ZoomFactor};

use super::components::{GizmoArrow, GizmoRoot};
use super::types::Axis2D;

pub(super) fn setup(mut commands: Commands) {
    let arrow_length = CONFIG().get::<f32>("debug/gizmo/arrow_length");
    let arrow_thickness = CONFIG().get::<f32>("debug/gizmo/arrow_thickness");
    let z = CONFIG().get::<f32>("debug/gizmo/z");

    // Gizmo Arrows – one entity, hidden until needed
    let half_arrow_size_x = Vec2::new(arrow_length, arrow_thickness) / 2.0;
    let half_arrow_size_y = Vec2::new(arrow_thickness, arrow_length) / 2.0;

    commands
        .spawn((Transform::default(), Visibility::Hidden, Name::new("Gizmo Root"), GizmoRoot))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: Color::linear_rgba(1.0, 0.0, 0.0, 0.9),
                    rect: Some(Rect::new(-half_arrow_size_x.x, -half_arrow_size_x.y, half_arrow_size_x.x, half_arrow_size_x.y)),
                    ..Default::default()
                },
                Meta::<Sprite>::default(),
                Transform::from_translation(Vec3::new(half_arrow_size_x.x + half_arrow_size_x.y, 0.0, z)),
                GizmoArrow { axis: Axis2D::X },
            ));

            parent.spawn((
                Sprite {
                    color: Color::linear_rgba(0.0, 1.0, 0.0, 0.9),
                    rect: Some(Rect::new(-half_arrow_size_y.x, -half_arrow_size_y.y, half_arrow_size_y.x, half_arrow_size_y.y)),
                    ..Default::default()
                },
                Meta::<Sprite>::default(),
                Transform::from_translation(Vec3::new(0.0, half_arrow_size_y.x + half_arrow_size_y.y, z)),
                GizmoArrow { axis: Axis2D::Y },
            ));
        });
}

pub(super) fn update_gizmo_visibility_and_position(
    mut gizmo_root: Query<(&mut Transform, &mut Visibility), With<GizmoRoot>>,
    transforms: Query<&GlobalTransform>,
    debug_suite_ui_state: Res<PrimaryWindowUiState>,
    zoom_factor: Res<ZoomFactor>,
) {
    let Ok((mut gizmo_transform, mut vis)) = gizmo_root.single_mut() else {
        return;
    };
    let selected = &debug_suite_ui_state.selected_entities;

    if selected.is_empty() {
        *vis = Visibility::Hidden;
        return;
    }

    *vis = Visibility::Visible;

    // Center on average position
    let mut avg = Vec3::ZERO;
    let mut count = 0;

    for entity in selected.iter() {
        if let Ok(transform) = transforms.get(entity) {
            avg += transform.translation();
            count += 1;
        }
    }

    if count > 0 {
        avg /= count as f32;
        gizmo_transform.translation = avg;
    }

    gizmo_transform.scale = Vec2::splat(zoom_factor.0).extend(1.0);
}

pub(super) fn move_selected_with_gizmo(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    gizmo_parts: Query<(&GizmoArrow, &GlobalTransform)>,
    debug_suite_ui_state: Res<PrimaryWindowUiState>,
    fixed_time: Res<Time<Fixed>>,
    zoom_factor: Res<ZoomFactor>,
) {
    let selected = &debug_suite_ui_state.selected_entities;

    for event in drag_events.read() {
        if event.pointer_id != META_MOUSE_POINTER_ID {
            continue;
        }

        if let Ok((gizmo_arrow, _)) = gizmo_parts.get(event.target) {
            let axis = match gizmo_arrow.axis {
                Axis2D::X => Vec3::X,
                Axis2D::Y => Vec3::Y,
            };

            let delta = axis
                * event.delta.dot(axis.truncate().normalize_or_zero())
                * fixed_time.delta_secs()
                * CONFIG().get::<f32>("debug/gizmo/drag_speed")
                * zoom_factor.0;

            for entity in selected.iter() {
                if let Ok(mut transform) = transforms.get_mut(entity) {
                    transform.translation += Vec3::new(delta.x, -delta.y, delta.z);
                }
            }
        }
    }
}
