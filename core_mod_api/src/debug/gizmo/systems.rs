use crate::bevy::prelude::*;

use crate::config::statics::CONFIG;
use crate::core::components::Meta;
use crate::picking::constants::META_MOUSE_POINTER_ID;
use crate::player::components::PlayerVisual3dLink;
use crate::render::components::{EntityProxyLink, MainCamera};
use crate::render::resources::{PrimaryWindowUiState, ZoomFactor};

use super::components::{GizmoArrow, GizmoRoot};
use super::types::Axis3D;

pub(super) fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let arrow_length = CONFIG().get::<f32>("debug/gizmo/arrow_length");
    let arrow_thickness = CONFIG().get::<f32>("debug/gizmo/arrow_thickness");
    let z = CONFIG().get::<f32>("debug/gizmo/z_offset");
    let head_length = (arrow_length * 0.35).max(arrow_thickness * 2.0);
    let shaft_length = (arrow_length - head_length).max(arrow_thickness * 1.5);
    let head_radius = (arrow_thickness * 1.8).max(arrow_thickness + 0.01);

    let shaft_mesh = meshes.add(Mesh::from(Cuboid::from_size(Vec3::new(arrow_thickness, shaft_length, arrow_thickness))));
    let head_mesh = meshes.add(Mesh::from(crate::bevy::math::primitives::Cone {
        radius: head_radius,
        height: head_length,
    }));

    let x_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(1.0, 0.2, 0.2, 0.95),
        unlit: true,
        ..Default::default()
    });
    let y_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(0.2, 1.0, 0.2, 0.95),
        unlit: true,
        ..Default::default()
    });
    let z_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(0.25, 0.55, 1.0, 0.95),
        unlit: true,
        ..Default::default()
    });

    let shaft_center = shaft_length * 0.5;
    let head_center = shaft_length + head_length * 0.5;

    commands
        .spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, z)),
            Visibility::Hidden,
            Name::new("Gizmo Root"),
            GizmoRoot,
        ))
        .with_children(|parent| {
            let x_rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::X);
            parent.spawn((
                Name::new("gizmo_axis_x_shaft"),
                Mesh3d(shaft_mesh.clone()),
                MeshMaterial3d(x_material.clone()),
                Meta::<Mesh3d>::default(),
                GizmoArrow { axis: Axis3D::X },
                Transform {
                    translation: Vec3::X * shaft_center,
                    rotation: x_rotation,
                    ..Default::default()
                },
            ));
            parent.spawn((
                Name::new("gizmo_axis_x_head"),
                Mesh3d(head_mesh.clone()),
                MeshMaterial3d(x_material),
                Meta::<Mesh3d>::default(),
                GizmoArrow { axis: Axis3D::X },
                Transform {
                    translation: Vec3::X * head_center,
                    rotation: x_rotation,
                    ..Default::default()
                },
            ));

            parent.spawn((
                Name::new("gizmo_axis_y_shaft"),
                Mesh3d(shaft_mesh.clone()),
                MeshMaterial3d(y_material.clone()),
                Meta::<Mesh3d>::default(),
                GizmoArrow { axis: Axis3D::Y },
                Transform::from_translation(Vec3::Y * shaft_center),
            ));
            parent.spawn((
                Name::new("gizmo_axis_y_head"),
                Mesh3d(head_mesh.clone()),
                MeshMaterial3d(y_material),
                Meta::<Mesh3d>::default(),
                GizmoArrow { axis: Axis3D::Y },
                Transform::from_translation(Vec3::Y * head_center),
            ));

            let z_rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::Z);
            parent.spawn((
                Name::new("gizmo_axis_z_shaft"),
                Mesh3d(shaft_mesh),
                MeshMaterial3d(z_material.clone()),
                Meta::<Mesh3d>::default(),
                GizmoArrow { axis: Axis3D::Z },
                Transform {
                    translation: Vec3::Z * shaft_center,
                    rotation: z_rotation,
                    ..Default::default()
                },
            ));
            parent.spawn((
                Name::new("gizmo_axis_z_head"),
                Mesh3d(head_mesh),
                MeshMaterial3d(z_material),
                Meta::<Mesh3d>::default(),
                GizmoArrow { axis: Axis3D::Z },
                Transform {
                    translation: Vec3::Z * head_center,
                    rotation: z_rotation,
                    ..Default::default()
                },
            ));
        });
}

pub(super) fn update_gizmo_visibility_and_position(
    mut gizmo_root: Query<(&mut Transform, &mut Visibility), With<GizmoRoot>>,
    transforms: Query<&GlobalTransform>,
    entity_proxy_links: Query<&EntityProxyLink>,
    player_visual_links: Query<&PlayerVisual3dLink>,
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

    let mut position_sum = Vec3::ZERO;
    let mut count = 0;

    for entity in selected.iter() {
        if let Some(position) = resolve_gizmo_anchor_position(entity, &transforms, &entity_proxy_links, &player_visual_links) {
            position_sum += position;
            count += 1;
        }
    }

    if count == 0 {
        *vis = Visibility::Hidden;
        return;
    }

    *vis = Visibility::Visible;
    let mut avg = position_sum / count as f32;
    avg.z += CONFIG().get::<f32>("debug/gizmo/z_offset");
    gizmo_transform.translation = avg;
    gizmo_transform.scale = Vec3::splat(zoom_factor.0.max(f32::EPSILON));
}

pub(super) fn move_selected_with_gizmo(
    mut drag_messages: MessageReader<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    gizmo_parts: Query<(&GizmoArrow, &GlobalTransform)>,
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    debug_suite_ui_state: Res<PrimaryWindowUiState>,
    fixed_time: Res<Time<Fixed>>,
    zoom_factor: Res<ZoomFactor>,
) {
    let selected = &debug_suite_ui_state.selected_entities;

    for message in drag_messages.read() {
        if message.pointer_id != META_MOUSE_POINTER_ID {
            continue;
        }

        if let Ok((gizmo_arrow, gizmo_transform)) = gizmo_parts.get(message.event_target()) {
            let axis = match gizmo_arrow.axis {
                Axis3D::X => Vec3::X,
                Axis3D::Y => Vec3::Y,
                Axis3D::Z => Vec3::Z,
            };

            let axis_screen_dir = main_camera
                .single()
                .ok()
                .and_then(|(camera, camera_transform)| {
                    let axis_origin = gizmo_transform.translation();
                    let axis_tip = axis_origin + axis;
                    let projected_axis = camera
                        .world_to_viewport(camera_transform, axis_origin)
                        .ok()
                        .zip(camera.world_to_viewport(camera_transform, axis_tip).ok())
                        .map(|(origin_screen, tip_screen)| (tip_screen - origin_screen).normalize_or_zero())
                        .filter(|dir| dir.length_squared() > f32::EPSILON);

                    projected_axis.or_else(|| {
                        let axis_view = camera_transform.affine().inverse().transform_vector3(axis);
                        let mut fallback = Vec2::new(axis_view.x, -axis_view.y);
                        if fallback.length_squared() <= f32::EPSILON {
                            let depth_sign = axis_view.z.signum();
                            fallback = Vec2::new(0.0, if depth_sign == 0.0 { -1.0 } else { -depth_sign });
                        }
                        (fallback.length_squared() > f32::EPSILON).then_some(fallback.normalize_or_zero())
                    })
                })
                .unwrap_or_else(|| {
                    let fallback = Vec2::new(axis.x, -axis.y);
                    if fallback.length_squared() <= f32::EPSILON {
                        Vec2::NEG_Y
                    } else {
                        fallback.normalize_or_zero()
                    }
                });

            if axis_screen_dir.length_squared() <= f32::EPSILON {
                continue;
            }

            let axis_delta = message.delta.dot(axis_screen_dir);
            let delta = axis * axis_delta * fixed_time.delta_secs() * CONFIG().get::<f32>("debug/gizmo/drag_speed") * zoom_factor.0;

            for entity in selected.iter() {
                if let Ok(mut transform) = transforms.get_mut(entity) {
                    transform.translation += delta;
                }
            }
        }
    }
}

fn resolve_gizmo_anchor_position(
    entity: Entity,
    transforms: &Query<&GlobalTransform>,
    entity_proxy_links: &Query<&EntityProxyLink>,
    player_visual_links: &Query<&PlayerVisual3dLink>,
) -> Option<Vec3> {
    if let Ok(link) = entity_proxy_links.get(entity) {
        if let Ok(transform) = transforms.get(link.render_entity) {
            return Some(transform.translation());
        }
        if let Ok(transform) = transforms.get(link.logic_entity) {
            return Some(transform.translation());
        }
    }

    if let Ok(link) = player_visual_links.get(entity)
        && let Ok(transform) = transforms.get(link.entity)
    {
        return Some(transform.translation());
    }

    transforms.get(entity).ok().map(|transform| transform.translation())
}
