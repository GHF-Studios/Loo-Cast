use crate::bevy::picking::prelude::Pickable;
use crate::bevy::prelude::*;

use crate::config::statics::CONFIG;
use crate::core::components::Meta;
use crate::picking::constants::{DIEGETIC_MOUSE_POINTER_ID, META_MOUSE_POINTER_ID, NO_HIT_SENTINEL};
use crate::player::components::{Player, PlayerVisual3dLink};
use crate::render::components::{EntityProxyLink, LogicProxy, MainCamera, RenderProxy};
use crate::render::resources::PrimaryWindowUiState;
use crate::chunk::components::{Chunk, ChunkActor, ChunkLoader};

use super::components::{GizmoArrow, GizmoRoot};
use super::types::Axis3D;

const GIZMO_DEPTH_BIAS: f32 = 100_000.0;

#[derive(Resource)]
pub(super) struct GizmoMaterialHandles {
    x: Handle<StandardMaterial>,
    y: Handle<StandardMaterial>,
    z: Handle<StandardMaterial>,
}

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
        depth_bias: GIZMO_DEPTH_BIAS,
        ..Default::default()
    });
    let y_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(0.2, 1.0, 0.2, 0.95),
        unlit: true,
        depth_bias: GIZMO_DEPTH_BIAS,
        ..Default::default()
    });
    let z_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(0.25, 0.55, 1.0, 0.95),
        unlit: true,
        depth_bias: GIZMO_DEPTH_BIAS,
        ..Default::default()
    });

    let shaft_center = shaft_length * 0.5;
    let head_center = shaft_length + head_length * 0.5;

    commands.insert_resource(GizmoMaterialHandles {
        x: x_material.clone(),
        y: y_material.clone(),
        z: z_material.clone(),
    });

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
                Pickable {
                    should_block_lower: true,
                    ..Default::default()
                },
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
                Pickable {
                    should_block_lower: true,
                    ..Default::default()
                },
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
                Pickable {
                    should_block_lower: true,
                    ..Default::default()
                },
                Meta::<Mesh3d>::default(),
                GizmoArrow { axis: Axis3D::Y },
                Transform::from_translation(Vec3::Y * shaft_center),
            ));
            parent.spawn((
                Name::new("gizmo_axis_y_head"),
                Mesh3d(head_mesh.clone()),
                MeshMaterial3d(y_material),
                Pickable {
                    should_block_lower: true,
                    ..Default::default()
                },
                Meta::<Mesh3d>::default(),
                GizmoArrow { axis: Axis3D::Y },
                Transform::from_translation(Vec3::Y * head_center),
            ));

            let z_rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::Z);
            parent.spawn((
                Name::new("gizmo_axis_z_shaft"),
                Mesh3d(shaft_mesh),
                MeshMaterial3d(z_material.clone()),
                Pickable {
                    should_block_lower: true,
                    ..Default::default()
                },
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
                Pickable {
                    should_block_lower: true,
                    ..Default::default()
                },
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    gizmo_materials: Res<GizmoMaterialHandles>,
    transforms: Query<&GlobalTransform>,
    chunks: Query<(), With<Chunk>>,
    chunk_actors: Query<(), With<ChunkActor>>,
    players: Query<(), With<Player>>,
    entity_proxy_links: Query<&EntityProxyLink>,
    render_proxies: Query<&RenderProxy>,
    logic_proxies: Query<&LogicProxy>,
    player_visual_links: Query<&PlayerVisual3dLink>,
    player_loader_query: Query<&ChunkLoader, With<Player>>,
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

    let selection_movable = selection_has_movable_targets(
        selected.iter(),
        &chunks,
        &chunk_actors,
        &players,
        &entity_proxy_links,
        &render_proxies,
        &logic_proxies,
    );
    apply_gizmo_material_state(selection_movable, &gizmo_materials, &mut materials);

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
    let zoom = player_loader_query
        .single()
        .ok()
        .map(|loader| loader.usf_transform.scale.local_f32())
        .unwrap_or(1.0)
        .max(0.001);
    let gizmo_scale = (1.0 / zoom).clamp(0.25, 100.0);
    gizmo_transform.scale = Vec3::splat(gizmo_scale);
}

pub(super) fn move_selected_with_gizmo(
    mut drag_messages: MessageReader<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    chunks: Query<(), With<Chunk>>,
    chunk_actors: Query<(), With<ChunkActor>>,
    players: Query<(), With<Player>>,
    entity_proxy_links: Query<&EntityProxyLink>,
    render_proxies: Query<&RenderProxy>,
    logic_proxies: Query<&LogicProxy>,
    gizmo_parts: Query<(&GizmoArrow, &GlobalTransform)>,
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    debug_suite_ui_state: Res<PrimaryWindowUiState>,
    player_loader_query: Query<&ChunkLoader, With<Player>>,
    fixed_time: Res<Time<Fixed>>,
) {
    let selected = &debug_suite_ui_state.selected_entities;
    let selection_movable = selection_has_movable_targets(
        selected.iter(),
        &chunks,
        &chunk_actors,
        &players,
        &entity_proxy_links,
        &render_proxies,
        &logic_proxies,
    );

    for message in drag_messages.read() {
        if message.pointer_id != META_MOUSE_POINTER_ID && message.pointer_id != DIEGETIC_MOUSE_POINTER_ID {
            continue;
        }

        let drag_target = message.event_target();
        if drag_target != NO_HIT_SENTINEL {
            warn!("gizmo_drag pointer={:?} target={:?} delta={:?}", message.pointer_id, drag_target, message.delta);
        }

        if let Ok((gizmo_arrow, gizmo_transform)) = gizmo_parts.get(drag_target) {
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
            let zoom = player_loader_query
                .single()
                .ok()
                .map(|loader| loader.usf_transform.scale.local_f32())
                .unwrap_or(1.0)
                .max(0.001);
            let motion_scale = (1.0 / zoom).clamp(0.1, 100.0);
            let world_delta = axis * axis_delta * fixed_time.delta_secs() * CONFIG().get::<f32>("debug/gizmo/drag_speed") * motion_scale;
            warn!(
                "gizmo_drag_apply axis={} pointer={:?} axis_delta={} world_delta={:?}",
                axis_label(&gizmo_arrow.axis),
                message.pointer_id,
                axis_delta,
                world_delta
            );

            if !selection_movable {
                continue;
            }

            for entity in selected.iter() {
                let source_entity = resolve_motion_source_entity(entity, &render_proxies, &logic_proxies);
                let is_player = players.get(source_entity).is_ok();

                // USF authoritative entities must move via their logical coordinate component,
                // not by writing Transform directly. Player is the explicit exception.
                if !is_player {
                    if chunks.get(source_entity).is_ok() {
                        continue;
                    }

                    if chunk_actors.get(source_entity).is_ok() {
                        continue;
                    }

                    if let Ok(link) = entity_proxy_links.get(source_entity)
                        && link.root_transform_is_proxy
                    {
                        warn!("gizmo_drag_skip_transform source={:?} reason=root_transform_is_proxy", source_entity);
                        continue;
                    }
                }

                if let Ok(mut transform) = transforms.get_mut(source_entity) {
                    transform.translation += world_delta;
                }
            }
        } else if drag_target != NO_HIT_SENTINEL {
            warn!("gizmo_drag target {:?} is not a GizmoArrow", drag_target);
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

fn resolve_motion_source_entity(entity: Entity, render_proxies: &Query<&RenderProxy>, logic_proxies: &Query<&LogicProxy>) -> Entity {
    if let Ok(render_proxy) = render_proxies.get(entity) {
        return render_proxy.source;
    }
    if let Ok(logic_proxy) = logic_proxies.get(entity) {
        return logic_proxy.source;
    }
    entity
}

fn selection_has_movable_targets<'a, I>(
    selected: I,
    chunks: &Query<(), With<Chunk>>,
    chunk_actors: &Query<(), With<ChunkActor>>,
    players: &Query<(), With<Player>>,
    entity_proxy_links: &Query<&EntityProxyLink>,
    render_proxies: &Query<&RenderProxy>,
    logic_proxies: &Query<&LogicProxy>,
) -> bool
where
    I: IntoIterator<Item = Entity>,
{
    for entity in selected {
        let source_entity = resolve_motion_source_entity(entity, render_proxies, logic_proxies);
        if players.get(source_entity).is_ok() {
            return true;
        }
        if chunks.get(source_entity).is_ok() || chunk_actors.get(source_entity).is_ok() {
            continue;
        }
        if let Ok(link) = entity_proxy_links.get(source_entity)
            && link.root_transform_is_proxy
        {
            continue;
        }
        return true;
    }

    false
}

fn apply_gizmo_material_state(enabled: bool, gizmo_materials: &GizmoMaterialHandles, materials: &mut Assets<StandardMaterial>) {
    if let Some(material) = materials.get_mut(&gizmo_materials.x) {
        material.base_color = gizmo_axis_color(Axis3D::X, enabled);
    }
    if let Some(material) = materials.get_mut(&gizmo_materials.y) {
        material.base_color = gizmo_axis_color(Axis3D::Y, enabled);
    }
    if let Some(material) = materials.get_mut(&gizmo_materials.z) {
        material.base_color = gizmo_axis_color(Axis3D::Z, enabled);
    }
}

fn gizmo_axis_color(axis: Axis3D, enabled: bool) -> Color {
    if enabled {
        match axis {
            Axis3D::X => Color::linear_rgba(1.0, 0.2, 0.2, 0.95),
            Axis3D::Y => Color::linear_rgba(0.2, 1.0, 0.2, 0.95),
            Axis3D::Z => Color::linear_rgba(0.25, 0.55, 1.0, 0.95),
        }
    } else {
        match axis {
            Axis3D::X => Color::linear_rgba(0.62, 0.42, 0.42, 0.45),
            Axis3D::Y => Color::linear_rgba(0.42, 0.62, 0.42, 0.45),
            Axis3D::Z => Color::linear_rgba(0.43, 0.5, 0.62, 0.45),
        }
    }
}

#[inline]
fn axis_label(axis: &Axis3D) -> &'static str {
    match axis {
        Axis3D::X => "X",
        Axis3D::Y => "Y",
        Axis3D::Z => "Z",
    }
}
