use bevy::prelude::*;


use super::components::GizmoArrow;
use crate::render::resources::PrimaryWindowUiState;
use super::types::Axis2D;

pub(super) fn setup(
    mut commands: Commands,
) {
    // Gizmo Arrows – one entity, hidden until needed
    let half_arrow_size_x = Vec2::new(5.0, 100.0) / 2.0;
    let half_arrow_size_y = Vec2::new(100.0, 5.0) / 2.0;

    commands
        .spawn((
            Transform::default(),
            Visibility::Hidden,
            Name::new("Gizmo Root"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: Color::linear_rgba(1.0, 0.0, 0.0, 1.0),
                    rect: Some(Rect::new(-half_arrow_size_x.x, -half_arrow_size_x.y, half_arrow_size_x.x, half_arrow_size_x.y)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(25.0, 0.0, 10.0)),
                GizmoArrow { axis: Axis2D::X },
            ));

            parent.spawn((
                Sprite {
                    color: Color::linear_rgba(0.0, 1.0, 0.0, 1.0),
                    rect: Some(Rect::new(-half_arrow_size_y.x, -half_arrow_size_y.y, half_arrow_size_y.x, half_arrow_size_y.y)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(0.0, 25.0, 10.0)),
                GizmoArrow { axis: Axis2D::Y },
            ));
        });
}

pub(super) fn update_gizmo_visibility_and_position(
    mut gizmo_root: Query<(&mut Transform, &mut Visibility), With<Children>>,
    transforms: Query<&GlobalTransform>,
    debug_suite_ui_state: Res<PrimaryWindowUiState>,
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
}

pub(super) fn move_selected_with_gizmo(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    gizmo_parts: Query<(&GizmoArrow, &GlobalTransform)>,
    debug_suite_ui_state: Res<PrimaryWindowUiState>,
) {
    let selected = &debug_suite_ui_state.selected_entities;

    for event in drag_events.read() {
        if let Ok((gizmo_arrow, _)) = gizmo_parts.get(event.target) {
            let axis = match gizmo_arrow.axis {
                Axis2D::X => Vec3::X,
                Axis2D::Y => Vec3::Y,
            };

            let delta = axis * event.delta.dot(axis.truncate().normalize_or_zero());

            for entity in selected.iter() {
                if let Ok(mut transform) = transforms.get_mut(entity) {
                    transform.translation += delta;
                }
            }
        }
    }
}
