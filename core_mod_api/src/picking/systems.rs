use crate::bevy::camera::primitives::Aabb;
use crate::bevy::camera::visibility::RenderLayers;
use crate::bevy::camera::{ImageRenderTarget, RenderTarget};
use crate::bevy::ecs::query::{QueryFilter, QuerySingleError};
use crate::bevy::input::ButtonState;
use crate::bevy::input::mouse::MouseWheel;
use crate::bevy::math::{Affine3A, Dir3, Ray3d, Vec3A, bounding::Aabb3d};
use crate::bevy::mesh::{Indices, PrimitiveTopology};
use crate::bevy::picking::PickingSettings;
use crate::bevy::picking::backend::prelude::*;
use crate::bevy::picking::input::PointerInputSettings;
use crate::bevy::picking::pointer::{Location, PointerAction, PointerButton, PointerId, PointerInput, PointerLocation, PointerPress};
use crate::bevy::prelude::*;
use crate::bevy::sprite::Anchor;
use crate::bevy::window::{PrimaryWindow, WindowEvent};

use crate::core::types::{Diegetic, Meta, OntologicalContext};
use crate::reflection::utils::functions::get_struct_field_mut;
use crate::render::{
    components::MainCamera,
    resources::{GameViewRenderTarget, PrimaryWindowUiState},
};

use super::constants::{DIEGETIC_MOUSE_POINTER_ID, META_MOUSE_POINTER_ID, NO_HIT_SENTINEL};
use super::resources::{SpritePickingMode, SpritePickingSettings};

pub(super) fn set_default_settings(mut pointer_input_settings: ResMut<PointerInputSettings>, mut picking_settings: ResMut<PickingSettings>) {
    *pointer_input_settings = PointerInputSettings {
        is_touch_enabled: false,
        is_mouse_enabled: false,
    };

    *picking_settings = PickingSettings {
        is_enabled: true,
        is_input_enabled: true,
        is_hover_enabled: true,
        is_window_picking_enabled: false,
    };
}

// TODO: Impl properly
pub(super) fn spawn_mouse_pointers(mut commands: Commands, game_view_render_target: Res<GameViewRenderTarget>) {
    commands.spawn((
        DIEGETIC_MOUSE_POINTER_ID,
        PointerLocation::new(Location {
            target: crate::bevy::camera::NormalizedRenderTarget::Image(ImageRenderTarget {
                handle: game_view_render_target.handle.clone(),
                scale_factor: 1.0,
            }),
            // TODO: Actually compute this
            position: Vec2::ZERO,
        }),
        PointerPress::default(),
    ));

    commands.spawn((
        META_MOUSE_POINTER_ID,
        PointerLocation::new(Location {
            target: crate::bevy::camera::NormalizedRenderTarget::Image(ImageRenderTarget {
                handle: game_view_render_target.handle.clone(),
                scale_factor: 1.0,
            }),
            // TODO: Actually compute this
            position: Vec2::ZERO,
        }),
        PointerPress::default(),
    ));
}

#[tracing::instrument(skip_all)]
pub(super) fn mouse_pick_messages(
    mut window_events: MessageReader<WindowEvent>,
    mut pointer_message_writer: MessageWriter<PointerInput>,
    mut cursor_last: Local<Vec2>,
    mut pointers: Query<(&PointerId, &mut PointerLocation, &mut PointerPress)>,
    primary_window: Query<(Entity, &Window), With<PrimaryWindow>>,
    debug_suite_ui_state: Res<PrimaryWindowUiState>,
    game_view_render_target: Res<GameViewRenderTarget>,
) {
    if window_events.is_empty() {
        return;
    }

    let Ok((primary_window_entity, primary_window)) = primary_window.single() else {
        return;
    };
    let Some(viewport) = debug_suite_ui_state.viewport_rect_precision_proxy else {
        return;
    };

    let Some(target) = RenderTarget::Image(ImageRenderTarget {
        handle: game_view_render_target.handle.clone(),
        scale_factor: 1.0,
    })
    .normalize(Some(primary_window_entity)) else {
        return;
    };

    let mut pointer_messages: Vec<PointerInput> = Vec::with_capacity(window_events.len());

    for window_event in window_events.read() {
        match window_event {
            WindowEvent::CursorMoved(message) => {
                let cursor_pos = message.position;
                let in_viewport = viewport.contains(egui::Pos2::new(cursor_pos.x, cursor_pos.y));
                let delta = cursor_pos - *cursor_last;
                *cursor_last = cursor_pos;
                if !in_viewport {
                    continue;
                }

                let location = Location {
                    target: target.clone(),
                    position: cursor_pos,
                };
                let action = PointerAction::Move { delta };
                pointer_messages.push(PointerInput::new(DIEGETIC_MOUSE_POINTER_ID, location.clone(), action.clone()));
                pointer_messages.push(PointerInput::new(META_MOUSE_POINTER_ID, location, action));
            }
            WindowEvent::MouseButtonInput(input) => {
                let cursor_pos = primary_window.cursor_position().unwrap_or(*cursor_last);
                let in_viewport = viewport.contains(egui::Pos2::new(cursor_pos.x, cursor_pos.y));
                *cursor_last = cursor_pos;

                let location = Location {
                    target: target.clone(),
                    position: cursor_pos,
                };
                let button = match input.button {
                    MouseButton::Left => PointerButton::Primary,
                    MouseButton::Right => PointerButton::Secondary,
                    MouseButton::Middle => PointerButton::Middle,
                    MouseButton::Other(_) | MouseButton::Back | MouseButton::Forward => continue,
                };
                let action = match input.state {
                    ButtonState::Pressed => PointerAction::Press(button),
                    ButtonState::Released => PointerAction::Release(button),
                };
                let should_emit = in_viewport || matches!(input.state, ButtonState::Released);
                if !should_emit {
                    continue;
                }

                pointer_messages.push(PointerInput::new(DIEGETIC_MOUSE_POINTER_ID, location.clone(), action.clone()));
                pointer_messages.push(PointerInput::new(META_MOUSE_POINTER_ID, location, action));
            }
            WindowEvent::MouseWheel(message) => {
                let cursor_pos = primary_window.cursor_position().unwrap_or(*cursor_last);
                let in_viewport = viewport.contains(egui::Pos2::new(cursor_pos.x, cursor_pos.y));
                *cursor_last = cursor_pos;
                if !in_viewport {
                    continue;
                }

                let MouseWheel { unit, x, y, window: _ } = *message;
                let location = Location {
                    target: target.clone(),
                    position: cursor_pos,
                };
                let action = PointerAction::Scroll { x, y, unit };
                pointer_messages.push(PointerInput::new(DIEGETIC_MOUSE_POINTER_ID, location.clone(), action.clone()));
                pointer_messages.push(PointerInput::new(META_MOUSE_POINTER_ID, location, action));
            }
            _ => {}
        }
    }

    for message in pointer_messages.into_iter() {
        pointers.iter_mut().for_each(|(pointer_id, mut pointer_location, mut pointer_press)| {
            if *pointer_id != message.pointer_id {
                return;
            }

            pointer_location.location = Some(message.location.clone());
            match &message.action {
                PointerAction::Press(button) => match button {
                    PointerButton::Primary => *get_struct_field_mut(&mut *pointer_press, "primary") = true,
                    PointerButton::Secondary => *get_struct_field_mut(&mut *pointer_press, "secondary") = true,
                    PointerButton::Middle => *get_struct_field_mut(&mut *pointer_press, "middle") = true,
                },
                PointerAction::Release(button) => match button {
                    PointerButton::Primary => *get_struct_field_mut(&mut *pointer_press, "primary") = false,
                    PointerButton::Secondary => *get_struct_field_mut(&mut *pointer_press, "secondary") = false,
                    PointerButton::Middle => *get_struct_field_mut(&mut *pointer_press, "middle") = false,
                },
                _ => {}
            }
        });

        pointer_message_writer.write(message);
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn sprite_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    main_camera_query: Query<(Entity, &Camera, &GlobalTransform, &Projection, Option<&RenderLayers>), With<MainCamera>>,
    diegetic_sprite_query: Query<
        (Entity, &Sprite, &GlobalTransform, &Anchor, &ViewVisibility, Option<&RenderLayers>),
        Without<crate::core::components::Meta<Sprite>>,
    >,
    meta_sprite_query: Query<(Entity, &Sprite, &GlobalTransform, &Anchor, &ViewVisibility, Option<&RenderLayers>), With<crate::core::components::Meta<Sprite>>>,
    diegetic_mesh_query: Query<
        (Entity, &Mesh3d, &GlobalTransform, &ViewVisibility, Option<&Aabb>, Option<&RenderLayers>),
        Without<crate::core::components::Meta<Mesh3d>>,
    >,
    meta_mesh_query: Query<
        (Entity, &Mesh3d, &GlobalTransform, &ViewVisibility, Option<&Aabb>, Option<&RenderLayers>),
        With<crate::core::components::Meta<Mesh3d>>,
    >,
    images: Res<Assets<Image>>,
    meshes: Res<Assets<Mesh>>,
    texture_atlas_layout: Res<Assets<TextureAtlasLayout>>,
    pickables: Query<&Pickable>,
    settings: Res<SpritePickingSettings>,
    game_view_render_target: Res<GameViewRenderTarget>,
    primary_window_ui_state: Res<PrimaryWindowUiState>,
    mut output: MessageWriter<PointerHits>,
) {
    let any_meta_hits = emit_context_hits(
        collect_context_hits::<Meta>(
            &pointers,
            &main_camera_query,
            &meta_sprite_query,
            &meta_mesh_query,
            &images,
            &meshes,
            &texture_atlas_layout,
            &pickables,
            &settings,
            &game_view_render_target,
            &primary_window_ui_state,
        ),
        &mut output,
    );

    if any_meta_hits {
        return;
    }

    let _any_diegetic_hits = emit_context_hits(
        collect_context_hits::<Diegetic>(
            &pointers,
            &main_camera_query,
            &diegetic_sprite_query,
            &diegetic_mesh_query,
            &images,
            &meshes,
            &texture_atlas_layout,
            &pickables,
            &settings,
            &game_view_render_target,
            &primary_window_ui_state,
        ),
        &mut output,
    );
}

#[derive(Debug)]
struct ContextPickResult {
    pointer_id: PointerId,
    camera_entity: Entity,
    order: f32,
    picks: Vec<(Entity, HitData)>,
}

fn emit_context_hits(result: Option<ContextPickResult>, output: &mut MessageWriter<PointerHits>) -> bool {
    let Some(mut result) = result else {
        return false;
    };

    if result.picks.is_empty() {
        result.picks.push((NO_HIT_SENTINEL, HitData::new(result.camera_entity, 0.0, None, None)));
        output.write(PointerHits::new(result.pointer_id, result.picks, result.order));
        return false;
    }

    output.write(PointerHits::new(result.pointer_id, result.picks, result.order));
    true
}

fn collect_context_hits<OC: OntologicalContext>(
    pointers: &Query<(&PointerId, &PointerLocation)>,
    main_camera_query: &Query<(Entity, &Camera, &GlobalTransform, &Projection, Option<&RenderLayers>), With<MainCamera>>,
    sprite_query: &Query<(Entity, &Sprite, &GlobalTransform, &Anchor, &ViewVisibility, Option<&RenderLayers>), OC::SpriteOntologyFilter>,
    mesh_query: &Query<(Entity, &Mesh3d, &GlobalTransform, &ViewVisibility, Option<&Aabb>, Option<&RenderLayers>), OC::MeshOntologyFilter>,
    images: &Res<Assets<Image>>,
    meshes: &Res<Assets<Mesh>>,
    texture_atlas_layout: &Res<Assets<TextureAtlasLayout>>,
    pickables: &Query<&Pickable>,
    settings: &Res<SpritePickingSettings>,
    game_view_render_target: &Res<GameViewRenderTarget>,
    primary_window_ui_state: &Res<PrimaryWindowUiState>,
) -> Option<ContextPickResult> {
    let (pointer_id, location) = match pointers.iter().find(|(p_id, _)| **p_id == OC::pointer_id()) {
        Some((pointer, pointer_location)) => match pointer_location.location().map(|loc| (*pointer, loc)) {
            Some(v) => v,
            None => {
                trace!("Mouse pointer is inactive");
                return None;
            }
        },
        None => {
            trace!("Mouse pointer not found");
            return None;
        }
    };

    let (main_camera_entity, main_camera, main_camera_transform, cursor_ray_len, camera_near_for_depth, camera_layers) = match main_camera_query.single() {
        Ok((ent, cam, cam_transform, cam_projection, cam_layers)) => {
            let (ray_len, near_for_depth) = match cam_projection {
                Projection::Orthographic(ortho) => ((ortho.far - ortho.near).abs(), ortho.near),
                Projection::Perspective(perspective) => (perspective.far.max(1.0), perspective.near),
                _ => {
                    debug!("Main camera projection is unsupported for picking");
                    return None;
                }
            };

            (ent, cam, cam_transform, ray_len, near_for_depth, cam_layers.cloned().unwrap_or_default())
        }
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                debug!("No main camera found");
                return None;
            }
            QuerySingleError::MultipleEntities(_) => panic!("Multiple MainCameras not supported!"),
        },
    };

    let viewport_size = game_view_render_target.size;
    let viewport_size_vec2 = Vec2::new(viewport_size.x as f32, viewport_size.y as f32);
    let current_window_position = location.position;
    let Some(viewport_rect) = primary_window_ui_state.viewport_rect_precision_proxy else {
        trace!("Viewport rect not found");
        return None;
    };

    if !viewport_rect.contains(egui::Pos2 {
        x: current_window_position.x,
        y: current_window_position.y,
    }) {
        trace!("Cursor outside viewport");
        return None;
    }

    let current_viewport_position = {
        let x = current_window_position
            .x
            .remap(viewport_rect.min.x, viewport_rect.max.x, 0.0, viewport_size_vec2.x);
        let y = current_window_position
            .y
            .remap(viewport_rect.min.y, viewport_rect.max.y, 0.0, viewport_size_vec2.y);
        Vec2::new(x, y)
    };

    let Ok(cursor_ray_world) = main_camera.viewport_to_world(main_camera_transform, current_viewport_position) else {
        debug!("Failed to compute cursor ray world position");
        return None;
    };

    let cursor_ray_end = cursor_ray_world.origin + cursor_ray_world.direction * cursor_ray_len;

    let mut candidates = collect_sprite_hits(
        sprite_query,
        images,
        texture_atlas_layout,
        settings,
        cursor_ray_world,
        cursor_ray_end,
        main_camera_entity,
        main_camera_transform,
        camera_near_for_depth,
        &camera_layers,
    );
    candidates.extend(collect_mesh_hits(
        mesh_query,
        meshes,
        cursor_ray_world,
        main_camera_entity,
        main_camera_transform,
        camera_near_for_depth,
        &camera_layers,
    ));

    candidates.sort_by(|(_, hit_a), (_, hit_b)| hit_a.depth.partial_cmp(&hit_b.depth).unwrap_or(std::cmp::Ordering::Equal));
    let picks = filter_pickable_hits(candidates, pickables);

    Some(ContextPickResult {
        pointer_id,
        camera_entity: main_camera_entity,
        order: main_camera.order as f32,
        picks,
    })
}

fn filter_pickable_hits(mut hits: Vec<(Entity, HitData)>, pickables: &Query<&Pickable>) -> Vec<(Entity, HitData)> {
    if hits.is_empty() {
        return hits;
    }

    let mut filtered_hits = Vec::with_capacity(hits.len());
    for (entity, hit) in hits.drain(..) {
        if let Ok(pickable) = pickables.get(entity) {
            if pickable.is_hoverable {
                filtered_hits.push((entity, hit));
            }
            if pickable.should_block_lower {
                break;
            }
        } else {
            filtered_hits.push((entity, hit));
            break;
        }
    }

    filtered_hits
}

fn collect_sprite_hits<OF: QueryFilter>(
    sprite_query: &Query<(Entity, &Sprite, &GlobalTransform, &Anchor, &ViewVisibility, Option<&RenderLayers>), OF>,
    images: &Res<Assets<Image>>,
    texture_atlas_layout: &Res<Assets<TextureAtlasLayout>>,
    settings: &Res<SpritePickingSettings>,
    cursor_ray_world: Ray3d,
    cursor_ray_end: Vec3,
    main_camera_entity: Entity,
    main_camera_transform: &GlobalTransform,
    camera_near_for_depth: f32,
    camera_layers: &RenderLayers,
) -> Vec<(Entity, HitData)> {
    let mut picks = Vec::new();

    for (entity, sprite, sprite_transform, anchor, vis, sprite_layers) in sprite_query.iter() {
        if !vis.get() || sprite_transform.affine().is_nan() {
            continue;
        }
        if !render_layers_intersect(camera_layers, sprite_layers) {
            continue;
        }

        // Transform cursor line segment to sprite coordinate system.
        let world_to_sprite = sprite_transform.affine().inverse();
        let cursor_start_sprite = world_to_sprite.transform_point3(cursor_ray_world.origin);
        let cursor_end_sprite = world_to_sprite.transform_point3(cursor_ray_end);

        // Find where the cursor segment intersects the plane Z=0 (the sprite plane in local space).
        if (cursor_start_sprite.z - cursor_end_sprite.z).abs() <= f32::EPSILON {
            continue;
        }
        let lerp_factor = f32::inverse_lerp(cursor_start_sprite.z, cursor_end_sprite.z, 0.0);
        if !(0.0..=1.0).contains(&lerp_factor) {
            continue;
        }

        let cursor_pos_sprite_3d = cursor_start_sprite.lerp(cursor_end_sprite, lerp_factor);
        let cursor_pos_sprite = Vec2::new(cursor_pos_sprite_3d.x, cursor_pos_sprite_3d.y);
        let Ok(cursor_pos_sprite_pixel) = sprite.compute_pixel_space_point(cursor_pos_sprite, *anchor, images, texture_atlas_layout) else {
            continue;
        };

        let cursor_in_valid_pixels_of_sprite = 'valid_pixel: {
            match settings.picking_mode {
                SpritePickingMode::AlphaThreshold(cutoff) => {
                    let Some(image) = images.get(&sprite.image) else {
                        // [`Sprite::from_color`] returns a defaulted handle. If the image is unavailable,
                        // we still allow this to count so color sprites remain pickable.
                        break 'valid_pixel true;
                    };

                    let Ok(color) = image.get_color_at(cursor_pos_sprite_pixel.x as u32, cursor_pos_sprite_pixel.y as u32) else {
                        break 'valid_pixel false;
                    };

                    color.alpha() > cutoff
                }
                SpritePickingMode::BoundingBox => true,
            }
        };

        if !cursor_in_valid_pixels_of_sprite {
            continue;
        }

        let hit_pos_world = sprite_transform.transform_point(cursor_pos_sprite_3d);
        let depth = compute_hit_depth(main_camera_transform, camera_near_for_depth, hit_pos_world);
        picks.push((
            entity,
            HitData::new(main_camera_entity, depth, Some(hit_pos_world), Some(*sprite_transform.back())),
        ));
    }

    picks
}

fn collect_mesh_hits<OF: QueryFilter>(
    mesh_query: &Query<(Entity, &Mesh3d, &GlobalTransform, &ViewVisibility, Option<&Aabb>, Option<&RenderLayers>), OF>,
    meshes: &Res<Assets<Mesh>>,
    cursor_ray_world: Ray3d,
    main_camera_entity: Entity,
    main_camera_transform: &GlobalTransform,
    camera_near_for_depth: f32,
    camera_layers: &RenderLayers,
) -> Vec<(Entity, HitData)> {
    let mut picks = Vec::new();

    for (entity, mesh_3d, mesh_transform, visibility, mesh_aabb, mesh_layers) in mesh_query.iter() {
        if !visibility.get() || mesh_transform.affine().is_nan() {
            continue;
        }
        if !render_layers_intersect(camera_layers, mesh_layers) {
            continue;
        }

        let mesh_affine = mesh_transform.affine();
        if let Some(aabb) = mesh_aabb {
            let mesh_aabb = Aabb3d::new(aabb.center, aabb.half_extents);
            if ray_aabb_intersection_3d(cursor_ray_world, &mesh_aabb, &mesh_affine).is_none() {
                continue;
            }
        }

        let Some(mesh) = meshes.get(&mesh_3d.0) else {
            continue;
        };

        let Some(intersection) = ray_intersection_over_mesh(mesh, &mesh_affine, cursor_ray_world) else {
            continue;
        };

        let depth = compute_hit_depth(main_camera_transform, camera_near_for_depth, intersection.point_world);
        picks.push((
            entity,
            HitData::new(main_camera_entity, depth, Some(intersection.point_world), Some(intersection.normal_world)),
        ));
    }

    picks
}

fn render_layers_intersect(camera_layers: &RenderLayers, entity_layers: Option<&RenderLayers>) -> bool {
    let entity_layers = entity_layers.cloned().unwrap_or_default();
    camera_layers.intersects(&entity_layers)
}

fn compute_hit_depth(main_camera_transform: &GlobalTransform, camera_near_for_depth: f32, hit_pos_world: Vec3) -> f32 {
    let hit_pos_cam = main_camera_transform.affine().inverse().transform_point3(hit_pos_world);
    -camera_near_for_depth - hit_pos_cam.z
}

#[derive(Debug, Clone, Copy)]
struct MeshIntersectionHit {
    point_world: Vec3,
    normal_world: Vec3,
}

fn ray_intersection_over_mesh(mesh: &Mesh, mesh_transform: &Affine3A, ray_world: Ray3d) -> Option<MeshIntersectionHit> {
    if mesh.primitive_topology() != PrimitiveTopology::TriangleList {
        return None;
    }

    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|attribute| attribute.as_float3())?;

    let world_to_mesh = mesh_transform.inverse();
    let ray_local = Ray3d::new(
        world_to_mesh.transform_point3(ray_world.origin),
        Dir3::new(world_to_mesh.transform_vector3(*ray_world.direction)).ok()?,
    );

    let mut closest_distance = f32::MAX;
    let mut closest_triangle: Option<[Vec3; 3]> = None;

    match mesh.indices() {
        Some(Indices::U16(indices)) => {
            ray_intersection_over_indexed_tris(&ray_local, positions, indices, &mut closest_distance, &mut closest_triangle);
        }
        Some(Indices::U32(indices)) => {
            ray_intersection_over_indexed_tris(&ray_local, positions, indices, &mut closest_distance, &mut closest_triangle);
        }
        None => {
            for tri in positions.chunks_exact(3) {
                let tri_vertices = [Vec3::from_array(tri[0]), Vec3::from_array(tri[1]), Vec3::from_array(tri[2])];
                if let Some(distance) = ray_triangle_intersection(&ray_local, &tri_vertices)
                    && distance >= 0.0
                    && distance < closest_distance
                {
                    closest_distance = distance;
                    closest_triangle = Some(tri_vertices);
                }
            }
        }
    }

    let closest_triangle = closest_triangle?;
    let point_local = ray_local.get_point(closest_distance);
    let normal_local = (closest_triangle[1] - closest_triangle[0]).cross(closest_triangle[2] - closest_triangle[0]);
    if normal_local.length_squared() <= f32::EPSILON {
        return None;
    }

    let point_world = mesh_transform.transform_point3(point_local);
    let normal_world = mesh_transform.transform_vector3(normal_local).normalize_or_zero();
    if normal_world.length_squared() <= f32::EPSILON {
        return None;
    }

    Some(MeshIntersectionHit { point_world, normal_world })
}

fn ray_intersection_over_indexed_tris<I: Copy + TryInto<usize>>(
    ray_local: &Ray3d,
    positions: &[[f32; 3]],
    indices: &[I],
    closest_distance: &mut f32,
    closest_triangle: &mut Option<[Vec3; 3]>,
) {
    for tri in indices.chunks_exact(3) {
        let [Ok(a), Ok(b), Ok(c)] = [tri[0].try_into(), tri[1].try_into(), tri[2].try_into()] else {
            continue;
        };
        let [Some(a), Some(b), Some(c)] = [positions.get(a), positions.get(b), positions.get(c)] else {
            continue;
        };

        let tri_vertices = [Vec3::from_array(*a), Vec3::from_array(*b), Vec3::from_array(*c)];
        if let Some(distance) = ray_triangle_intersection(ray_local, &tri_vertices)
            && distance >= 0.0
            && distance < *closest_distance
        {
            *closest_distance = distance;
            *closest_triangle = Some(tri_vertices);
        }
    }
}

fn ray_triangle_intersection(ray: &Ray3d, triangle: &[Vec3; 3]) -> Option<f32> {
    // Moller-Trumbore with backface culling.
    let edge_01 = triangle[1] - triangle[0];
    let edge_02 = triangle[2] - triangle[0];
    let p_vec = ray.direction.cross(edge_02);
    let determinant = edge_01.dot(p_vec);
    if determinant <= f32::EPSILON {
        return None;
    }

    let determinant_inverse = 1.0 / determinant;
    let t_vec = ray.origin - triangle[0];
    let u = t_vec.dot(p_vec) * determinant_inverse;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q_vec = t_vec.cross(edge_01);
    let v = (*ray.direction).dot(q_vec) * determinant_inverse;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    Some(edge_02.dot(q_vec) * determinant_inverse)
}

fn ray_aabb_intersection_3d(ray: Ray3d, aabb: &Aabb3d, model_to_world: &Affine3A) -> Option<f32> {
    // Transform the ray to model space so we can intersect against the mesh-local AABB.
    let world_to_model = model_to_world.inverse();
    let ray_direction: Vec3A = world_to_model.transform_vector3a((*ray.direction).into());
    let ray_direction_recip = ray_direction.recip();
    let ray_origin: Vec3A = world_to_model.transform_point3a(ray.origin.into());

    let positive = ray_direction.signum().cmpgt(Vec3A::ZERO);
    let min = Vec3A::select(positive, aabb.min, aabb.max);
    let max = Vec3A::select(positive, aabb.max, aabb.min);
    let tmin = (min - ray_origin) * ray_direction_recip;
    let tmax = (max - ray_origin) * ray_direction_recip;

    let tmin = tmin.max_element().max(0.0);
    let tmax = tmax.min_element();

    if tmin <= tmax { Some(tmin) } else { None }
}
