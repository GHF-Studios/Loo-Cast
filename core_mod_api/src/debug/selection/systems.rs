use crate::bevy::prelude::*;
use crate::bevy::sprite::Anchor;

use crate::core::components::Meta;
use crate::picking::constants::{DIEGETIC_MOUSE_POINTER_ID, NO_HIT_SENTINEL};
use crate::player::components::PlayerVisual3dLink;
use crate::render::components::{EntityProxyLink, LogicProxy, RenderProxy};
use crate::render::resources::PrimaryWindowUiState;
use std::collections::HashSet;

#[tracing::instrument(skip_all)]
pub(super) fn handle_selection(
    mut messages: MessageReader<Pointer<Click>>,
    selectable_query: Query<Entity, (Or<(With<Sprite>, With<Mesh3d>)>, Without<Meta<Sprite>>, Without<Meta<Mesh3d>>)>,
    render_proxy_query: Query<&RenderProxy>,
    logic_proxy_query: Query<&LogicProxy>,
    player_visual_link_query: Query<&PlayerVisual3dLink>,
    child_of_query: Query<&ChildOf>,
    mut debug_suite_ui_state: ResMut<PrimaryWindowUiState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let remap_pick_targets = debug_suite_ui_state.remap_pick_targets_to_source_entities;
    let selected = &mut debug_suite_ui_state.selected_entities;

    for message in messages.read() {
        if message.pointer_id != DIEGETIC_MOUSE_POINTER_ID {
            continue;
        }

        let clicked_target = message.event_target();

        if clicked_target == NO_HIT_SENTINEL {
            warn!("Clicked on empty space; clearing selection.");
            selected.clear();
            continue;
        }

        let target = if remap_pick_targets {
            remap_pick_target_to_source_entity(
                clicked_target,
                &render_proxy_query,
                &logic_proxy_query,
                &player_visual_link_query,
                &child_of_query,
            )
        } else {
            clicked_target
        };

        if remap_pick_targets && target != clicked_target {
            warn!("Remapped picked target {:?} -> {:?}", clicked_target, target);
        }

        if selectable_query.get(target).is_err() {
            warn!("Tried to select non-existent/incompatible entity: {:?}", target);
            continue;
        }

        warn!("Selecting entity: {:?}", target);

        let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);

        if ctrl {
            if selected.contains(target) {
                selected.retain(|e| e != target);
            } else {
                selected.select_maybe_add(target, true);
            }
        } else {
            selected.select_replace(target);
        }
    }
}

fn remap_pick_target_to_source_entity(
    picked_entity: Entity,
    render_proxy_query: &Query<&RenderProxy>,
    logic_proxy_query: &Query<&LogicProxy>,
    player_visual_link_query: &Query<&PlayerVisual3dLink>,
    child_of_query: &Query<&ChildOf>,
) -> Entity {
    if let Ok(render_proxy) = render_proxy_query.get(picked_entity) {
        return render_proxy.source;
    }
    if let Ok(logic_proxy) = logic_proxy_query.get(picked_entity) {
        return logic_proxy.source;
    }

    let mut current = picked_entity;
    for _ in 0..32 {
        let Ok(child_of) = child_of_query.get(current) else {
            break;
        };
        let parent = child_of.parent();

        if let Ok(render_proxy) = render_proxy_query.get(parent) {
            return render_proxy.source;
        }
        if let Ok(logic_proxy) = logic_proxy_query.get(parent) {
            return logic_proxy.source;
        }
        if let Ok(visual_link) = player_visual_link_query.get(parent)
            && visual_link.entity == current
        {
            return parent;
        }

        current = parent;
    }

    picked_entity
}

#[tracing::instrument(skip_all)]
pub(super) fn draw_selection_highlight_gizmos(
    mut gizmos: Gizmos,
    time: Res<Time<Real>>,
    debug_suite_ui_state: Res<PrimaryWindowUiState>,
    entity_proxy_links: Query<&EntityProxyLink>,
    player_visual_links: Query<&PlayerVisual3dLink>,
    children_query: Query<&Children>,
    sprite_query: Query<(&Sprite, &Anchor, &GlobalTransform), Without<Meta<Sprite>>>,
    mesh_query: Query<(&Mesh3d, &GlobalTransform, Option<&crate::bevy::camera::primitives::Aabb>), Without<Meta<Mesh3d>>>,
    meshes: Res<Assets<Mesh>>,
    images: Res<Assets<Image>>,
) {
    let selected = &debug_suite_ui_state.selected_entities;
    if selected.is_empty() {
        return;
    }

    let mut expansion_roots: Vec<Entity> = Vec::with_capacity(selected.len() * 4);
    for entity in selected.iter() {
        expansion_roots.push(entity);

        if let Ok(link) = entity_proxy_links.get(entity) {
            expansion_roots.push(link.render_entity);
        }
        if let Ok(link) = player_visual_links.get(entity) {
            expansion_roots.push(link.entity);
        }
    }

    let mut highlight_targets: HashSet<Entity> = HashSet::with_capacity(expansion_roots.len() * 2);
    for root in expansion_roots {
        collect_entity_and_descendants(root, &children_query, &mut highlight_targets);
    }

    let pulse = 0.5 + 0.5 * (time.elapsed_secs() * 5.0).sin();
    let sprite_color = Color::linear_rgba(1.0, 0.94, 0.3, 0.6 + 0.25 * pulse);
    let mesh_color = Color::linear_rgba(0.35, 0.92, 1.0, 0.55 + 0.3 * pulse);

    for entity in highlight_targets {
        if let Ok((sprite, anchor, transform)) = sprite_query.get(entity) {
            draw_sprite_outline(&mut gizmos, sprite, *anchor, transform, &images, sprite_color);
        }

        if let Ok((mesh_3d, transform, maybe_aabb)) = mesh_query.get(entity) {
            let aabb = maybe_aabb.copied().or_else(|| {
                use crate::bevy::camera::primitives::MeshAabb;
                meshes.get(&mesh_3d.0).and_then(|mesh| mesh.compute_aabb())
            });

            if let Some(aabb) = aabb {
                let (scale, rotation, _) = transform.to_scale_rotation_translation();
                let center_world = transform.transform_point(aabb.center.into());
                let world_scale = scale * (Vec3::from(aabb.half_extents) * 2.05);
                gizmos.cube(
                    Transform {
                        translation: center_world,
                        rotation,
                        scale: world_scale,
                    },
                    mesh_color,
                );
            }
        }
    }
}

fn collect_entity_and_descendants(entity: Entity, children_query: &Query<&Children>, out: &mut HashSet<Entity>) {
    if !out.insert(entity) {
        return;
    }

    let mut stack = vec![(entity, 0_u8)];
    while let Some((current, depth)) = stack.pop() {
        if depth >= 32 {
            continue;
        }

        let Ok(children) = children_query.get(current) else {
            continue;
        };

        for child in children.iter() {
            if out.insert(child) {
                stack.push((child, depth + 1));
            }
        }
    }
}

fn draw_sprite_outline(gizmos: &mut Gizmos, sprite: &Sprite, anchor: Anchor, transform: &GlobalTransform, images: &Assets<Image>, color: Color) {
    let sprite_size = sprite
        .rect
        .map(|rect| rect.size())
        .or(sprite.custom_size)
        .or_else(|| images.get(&sprite.image).map(|image| image.size_f32()))
        .unwrap_or(Vec2::splat(1.0));

    if sprite_size.x <= f32::EPSILON || sprite_size.y <= f32::EPSILON {
        return;
    }

    let half = sprite_size * 0.5;
    let center = -anchor.as_vec() * sprite_size;
    let corners_local = [
        Vec3::new(center.x - half.x, center.y - half.y, 0.0),
        Vec3::new(center.x + half.x, center.y - half.y, 0.0),
        Vec3::new(center.x + half.x, center.y + half.y, 0.0),
        Vec3::new(center.x - half.x, center.y + half.y, 0.0),
    ];

    let corners_world = corners_local.map(|corner| transform.transform_point(corner));
    for i in 0..4 {
        gizmos.line(corners_world[i], corners_world[(i + 1) % 4], color);
    }
}
