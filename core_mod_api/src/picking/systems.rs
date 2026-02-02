use crate::bevy::ecs::query::QuerySingleError;
use crate::bevy::input::mouse::MouseWheel;
use crate::bevy::input::ButtonState;
use crate::bevy::math::FloatOrd;
use crate::bevy::picking::PickingSettings;
use crate::bevy::picking::backend::prelude::*;
use crate::bevy::picking::input::PointerInputSettings;
use crate::bevy::picking::pointer::{Location, PointerAction, PointerButton, PointerId, PointerInput, PointerLocation, PointerPress};
use crate::bevy::prelude::*;
use crate::bevy::camera::{ImageRenderTarget, RenderTarget};
use crate::bevy::window::{PrimaryWindow, WindowEvent};

use crate::core::types::{Diegetic, Meta, OntologicalContext};
use crate::reflect::functions::get_struct_field_mut;
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
    let Some(cursor_pos) = primary_window.cursor_position() else { return };
    let Some(viewport) = debug_suite_ui_state.viewport_rect_precision_proxy else {
        return;
    };

    // Only inject pointer if it's within the egui image viewport
    if !viewport.contains(egui::Pos2::new(cursor_pos.x, cursor_pos.y)) {
        return;
    }

    let mut pointer_messages: Vec<PointerInput> = Vec::with_capacity(window_events.len());

    for window_event in window_events.read() {
        match window_event {
            WindowEvent::CursorMoved(message) => {
                let location = Location {
                    target: match RenderTarget::Image(ImageRenderTarget {
                        handle: game_view_render_target.handle.clone(),
                        scale_factor: 1.0,
                    })
                    .normalize(Some(primary_window_entity))
                    {
                        Some(target) => target,
                        None => continue,
                    },
                    position: message.position,
                };
                let action = PointerAction::Move {
                    delta: message.position - *cursor_last,
                };
                pointer_messages.push(PointerInput::new(DIEGETIC_MOUSE_POINTER_ID, location.clone(), action));
                pointer_messages.push(PointerInput::new(META_MOUSE_POINTER_ID, location, action));
                *cursor_last = message.position;
            }
            WindowEvent::MouseButtonInput(input) => {
                let location = Location {
                    target: match RenderTarget::Image(ImageRenderTarget {
                        handle: game_view_render_target.handle.clone(),
                        scale_factor: 1.0,
                    })
                    .normalize(Some(primary_window_entity))
                    {
                        Some(target) => target,
                        None => continue,
                    },
                    position: *cursor_last,
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
                pointer_messages.push(PointerInput::new(DIEGETIC_MOUSE_POINTER_ID, location.clone(), action));
                pointer_messages.push(PointerInput::new(META_MOUSE_POINTER_ID, location, action));
            }
            WindowEvent::MouseWheel(message) => {
                let MouseWheel { unit, x, y, window: _ } = *message;
                let location = Location {
                    target: match RenderTarget::Image(ImageRenderTarget {
                        handle: game_view_render_target.handle.clone(),
                        scale_factor: 1.0,
                    })
                    .normalize(Some(primary_window_entity))
                    {
                        Some(target) => target,
                        None => continue,
                    },
                    position: *cursor_last,
                };
                let action = PointerAction::Scroll { x, y, unit };
                pointer_messages.push(PointerInput::new(DIEGETIC_MOUSE_POINTER_ID, location.clone(), action));
                pointer_messages.push(PointerInput::new(META_MOUSE_POINTER_ID, location, action));
            }
            _ => {}
        }
    }

    for message in pointer_messages.into_iter() {
        match message.action {
            PointerAction::Press(ref button) => {
                pointers.iter_mut().for_each(|(pointer_id, _, mut pointer)| {
                    if *pointer_id == message.pointer_id {
                        match button {
                            // We utilize reflection here because the `PointerPress` struct was never meant to be directly mutable outside of Bevy Picking's internal systems (But I don't care hehe)
                            PointerButton::Primary => *get_struct_field_mut(&mut *pointer, "primary") = true,
                            PointerButton::Secondary => *get_struct_field_mut(&mut *pointer, "secondary") = true,
                            PointerButton::Middle => *get_struct_field_mut(&mut *pointer, "middle") = true,
                        }
                    }
                });
            }
            PointerAction::Release(ref button) => {
                pointers.iter_mut().for_each(|(pointer_id, _, mut pointer)| {
                    if *pointer_id == message.pointer_id {
                        match button {
                            // Same as above
                            PointerButton::Primary => *get_struct_field_mut(&mut *pointer, "primary") = false,
                            PointerButton::Secondary => *get_struct_field_mut(&mut *pointer, "secondary") = false,
                            PointerButton::Middle => *get_struct_field_mut(&mut *pointer, "middle") = false,
                        }
                    }
                });
            }
            PointerAction::Move { .. } => {
                pointers.iter_mut().for_each(|(id, mut pointer, _)| {
                    if *id == message.pointer_id {
                        pointer.location = Some(message.location.to_owned());
                    }
                });
            }
            _ => {}
        }

        pointer_message_writer.write(message);
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn sprite_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    main_camera_query: Query<(Entity, &Camera, &GlobalTransform, &Projection), With<MainCamera>>,
    diegetic_sprite_query: Query<(Entity, &Sprite, &GlobalTransform, &ViewVisibility), Without<crate::core::components::Meta<Sprite>>>,
    meta_sprite_query: Query<(Entity, &Sprite, &GlobalTransform, &ViewVisibility), With<crate::core::components::Meta<Sprite>>>,
    images: Res<Assets<Image>>,
    texture_atlas_layout: Res<Assets<TextureAtlasLayout>>,
    settings: Res<SpritePickingSettings>,
    game_view_render_target: Res<GameViewRenderTarget>,
    primary_window_ui_state: Res<PrimaryWindowUiState>,
    mut output: MessageWriter<PointerHits>,
) {
    let any_meta_hits = sprite_picking_backend_inner::<Meta>(
        &pointers,
        &main_camera_query,
        &meta_sprite_query,
        &images,
        &texture_atlas_layout,
        &settings,
        &game_view_render_target,
        &primary_window_ui_state,
        &mut output,
    );

    if any_meta_hits {
        return;
    }

    let _any_diegetic_hits = sprite_picking_backend_inner::<Diegetic>(
        &pointers,
        &main_camera_query,
        &diegetic_sprite_query,
        &images,
        &texture_atlas_layout,
        &settings,
        &game_view_render_target,
        &primary_window_ui_state,
        &mut output,
    );
}

fn sprite_picking_backend_inner<OC: OntologicalContext>(
    pointers: &Query<(&PointerId, &PointerLocation)>,
    main_camera_query: &Query<(Entity, &Camera, &GlobalTransform, &Projection), With<MainCamera>>,
    sprite_query: &Query<(Entity, &Sprite, &GlobalTransform, &ViewVisibility), OC::SpriteOntologyFilter>,
    images: &Res<Assets<Image>>,
    texture_atlas_layout: &Res<Assets<TextureAtlasLayout>>,
    settings: &Res<SpritePickingSettings>,
    game_view_render_target: &Res<GameViewRenderTarget>,
    primary_window_ui_state: &Res<PrimaryWindowUiState>,
    output: &mut MessageWriter<PointerHits>,
) -> bool {
    let (pointer_id, location) = match pointers.iter().find(|(p_id, _)| **p_id == OC::pointer_id()) {
        Some((pointer, pointer_location)) => match pointer_location.location().map(|loc| (pointer, loc)) {
            Some(v) => v,
            None => {
                warn!("Mouse pointer is inactive");
                return false;
            }
        },
        None => {
            warn!("Mouse pointer not found");
            return false;
        }
    };

    let (main_camera_entity, main_camera, main_camera_transform, main_camera_ortho) = match main_camera_query.single() {
        Ok((ent, cam, cam_transform, cam_projection)) => match cam_projection {
            Projection::Orthographic(ortho) => (ent, cam, cam_transform, ortho),
            _ => {
                warn!("Main camera is not orthographic");
                return false;
            }
        },
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                warn!("No main camera found");
                return false;
            }
            QuerySingleError::MultipleEntities(_) => panic!("Multiple MainCameras not supported!"),
        },
    };

    let mut sorted_sprites: Vec<_> = sprite_query
        .iter()
        .filter_map(|(entity, sprite, transform, vis)| {
            if !transform.affine().is_nan() && vis.get() {
                Some((entity, sprite, transform))
            } else {
                None
            }
        })
        .collect();

    // radsort is a stable radix sort that performed better than `slice::sort_by_key` (according to bevy's source code)
    radsort::sort_by_key(&mut sorted_sprites, |(_, _, transform)| -transform.translation().z);

    let mut blocked = false;
    let viewport_size = game_view_render_target.size;
    let viewport_size_vec2 = Vec2::new(viewport_size.x as f32, viewport_size.y as f32);
    let current_window_position = location.position;
    let Some(viewport_rect) = primary_window_ui_state.viewport_rect_precision_proxy else {
        warn!("Viewport rect not found");
        return false;
    };

    if !viewport_rect.contains(egui::Pos2 {
        x: current_window_position.x,
        y: current_window_position.y,
    }) {
        warn!("Cursor outside viewport");
        return false;
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

    // let viewport_pos = main_camera
    //     .logical_viewport_rect()
    //     .map(|v| v.min)
    //     .unwrap_or_default();
    // let pos_in_viewport = current_position - viewport_pos;

    let Ok(cursor_ray_world) = main_camera.viewport_to_world(main_camera_transform, current_viewport_position) else {
        warn!("Failed to compute cursor ray world position");
        return false;
    };

    let cursor_ray_len = main_camera_ortho.far - main_camera_ortho.near;
    let cursor_ray_end = cursor_ray_world.origin + cursor_ray_world.direction * cursor_ray_len;
    let mut picks: Vec<(Entity, HitData)> = sorted_sprites
        .iter()
        .copied()
        .filter_map(|(entity, sprite, sprite_transform)| {
            if blocked {
                // warn!("Picking for Entity {:?} blocked by previous sprite", entity);
                return None;
            }

            // Transform cursor line segment to sprite coordinate system
            let world_to_sprite = sprite_transform.affine().inverse();
            let cursor_start_sprite = world_to_sprite.transform_point3(cursor_ray_world.origin);
            let cursor_end_sprite = world_to_sprite.transform_point3(cursor_ray_end);

            // warn!(
            //     "Evaluating Entity {:?} — sprite Z: {:?}, world_to_sprite Z: {:?}",
            //     entity,
            //     sprite_transform.translation().z,
            //     sprite_transform
            //         .affine()
            //         .inverse()
            //         .transform_point3(cursor_ray_world.origin)
            //         .z,
            // );

            // Find where the cursor segment intersects the plane Z=0 (which is the sprite's
            // plane in sprite-local space). It may not intersect if, for example, we're
            // viewing the sprite side-on
            if cursor_start_sprite.z == cursor_end_sprite.z {
                // Cursor ray is parallel to the sprite and misses it
                warn!("Cursor ray parallel to sprite plane");
                return None;
            }
            let lerp_factor = f32::inverse_lerp(cursor_start_sprite.z, cursor_end_sprite.z, 0.0);
            if !(0.0..=1.0).contains(&lerp_factor) {
                // Lerp factor is out of range, meaning that while an infinite line cast by
                // the cursor would intersect the sprite, the sprite is not between the
                // camera's near and far planes

                warn!("Cursor ray does not intersect sprite plane within segment");
                return None;
            }
            // Otherwise we can interpolate the xy of the start and end positions by the
            // lerp factor to get the cursor position in sprite space!
            let cursor_pos_sprite = cursor_start_sprite.lerp(cursor_end_sprite, lerp_factor).xy();

            let Some(image) = images.get(&sprite.image) else {
                warn!("Sprite image not found");
                return None;
            };

            let sprite_size = sprite.custom_size.unwrap_or(Vec2::ONE)
                * image.size().as_vec2()
                * sprite.rect.unwrap_or_default().size();

            let cursor_pos_sprite_pixel = cursor_pos_sprite;

            // warn!(
            //     "Entity {:?} — Sprite world pos: {:?}, sprite_size: {:?}, cursor_pos_sprite: {:?}",
            //     entity,
            //     sprite_transform.translation(),
            //     sprite_size,
            //     cursor_pos_sprite
            // );

            let Ok(cursor_pos_sprite_pixel) = sprite.compute_pixel_space_point(
                cursor_pos_sprite_pixel,
                crate::bevy::sprite::Anchor(Vec2::ZERO),
                images,
                texture_atlas_layout
            ) else {
                // warn!("Cursor position '{}' outside sprite bounds", cursor_pos_sprite_pixel);
                return None;
            };

            // warn!(
            //     "→ cursor pixel space pos: {:?} (image size: {:?})",
            //     cursor_pos_sprite_pixel,
            //     image.size()
            // );

            let cursor_pos_sprite_pixel = cursor_pos_sprite_pixel - sprite_size / 2.0;

            // Since the pixel space coordinate is `Ok`, we know the cursor is in the bounds of
            // the sprite.

            let cursor_in_valid_pixels_of_sprite = 'valid_pixel: {
                match settings.picking_mode {
                    SpritePickingMode::AlphaThreshold(cutoff) => {
                        let Some(image) = images.get(&sprite.image) else {
                            // [`Sprite::from_color`] returns a defaulted handle.
                            // This handle doesn't return a valid image, so returning false here would make picking "color sprites" impossible
                            warn!("Sprite image not found");
                            break 'valid_pixel true;
                        };

                        let Ok(color) = image.get_color_at(cursor_pos_sprite_pixel.x as u32, cursor_pos_sprite_pixel.y as u32) else {
                            // We don't know how to interpret the pixel.
                            warn!("Failed to get color at cursor pixel space: {}", cursor_pos_sprite_pixel);
                            break 'valid_pixel false;
                        };

                        if color.alpha() > cutoff {
                            true
                        } else {
                            warn!("Alpha threshold '{}' was not met: {:?}", cutoff, color);
                            false
                        }
                    }
                    SpritePickingMode::BoundingBox => true,
                }
            };

            blocked = cursor_in_valid_pixels_of_sprite;

            cursor_in_valid_pixels_of_sprite.then(|| {
                let hit_pos_world = sprite_transform.transform_point(cursor_pos_sprite.extend(0.0));

                // Transform point from world to camera space to get the Z distance
                let hit_pos_cam = main_camera_transform.affine().inverse().transform_point3(hit_pos_world);

                // HitData requires a depth as calculated from the camera's near clipping plane
                let depth = -main_camera_ortho.near - hit_pos_cam.z;

                // warn!("✅ Picked entity {:?} at world Z: {:?}", entity, sprite_transform.translation().z);

                (
                    entity,
                    HitData::new(main_camera_entity, depth, Some(hit_pos_world), Some(*sprite_transform.back())),
                )
            })
        })
        .collect();

    if !picks.is_empty() {
        // warn!("Pick(s) detected for mouse pointer");

        let order = main_camera.order as f32;
        output.write(PointerHits::new(*pointer_id, picks, order));

        true
    } else {
        picks.push((NO_HIT_SENTINEL, HitData::new(main_camera_entity, 0.0, None, None)));

        let order = main_camera.order as f32;
        output.write(PointerHits::new(*pointer_id, picks, order));

        false
    }
}
